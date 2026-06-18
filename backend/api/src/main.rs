use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use domain::{
    ControlMethod, ControlledContent, ControlledContentRepository, ControlledContentType,
};
use persistence::sqlite::{SqliteChangelogRepository, SqliteControlledContentRepository, init_db};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

type ContentRepo = Arc<dyn ControlledContentRepository + Send + Sync>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://wiki.db?mode=rwc".to_owned());

    let pool = SqlitePool::connect(&db_url).await?;
    init_db(&pool).await?;

    let content_repo: ContentRepo = Arc::new(SqliteControlledContentRepository::new(pool.clone()));
    let changelog = Arc::new(SqliteChangelogRepository::new(pool.clone()));

    let app = build_app(content_repo.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("HTTP API listening on http://0.0.0.0:8080");
    let server = axum::serve(listener, app);

    // The bot is optional so the DB -> API -> page slice can run without a Telegram
    // token. Only start it when TELOXIDE_TOKEN is configured.
    if std::env::var("TELOXIDE_TOKEN").is_ok() {
        let bot = teloxide::Bot::from_env();
        tokio::select! {
            result = server => result?,
            _ = bot::run(bot, content_repo, changelog) => {}
        }
    } else {
        eprintln!("TELOXIDE_TOKEN not set — running HTTP API only (bot disabled).");
        server.await?;
    }

    Ok(())
}

fn build_app(content_repo: ContentRepo) -> Router {
    Router::new()
        .route("/api/controlled-content", get(list_controlled))
        .layer(CorsLayer::permissive())
        .with_state(content_repo)
}

#[derive(Serialize)]
struct ControlledDto {
    alias: String,
    #[serde(rename = "type")]
    content_type: String,
    status: String,
    reason: Option<String>,
}

#[derive(Deserialize)]
struct ListQuery {
    r#type: Option<String>,
}

async fn list_controlled(
    State(repo): State<ContentRepo>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<ControlledDto>>, StatusCode> {
    let types = match query.r#type.as_deref() {
        Some("artist") => vec![ControlledContentType::Artist],
        Some("character") => vec![ControlledContentType::Character],
        Some("tag") => vec![ControlledContentType::Tag],
        Some(_) => return Err(StatusCode::BAD_REQUEST),
        None => vec![
            ControlledContentType::Artist,
            ControlledContentType::Character,
            ControlledContentType::Tag,
        ],
    };

    let mut out = Vec::new();
    for content_type in types {
        let items = repo
            .find_all_controlled_by(content_type)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        out.extend(items.into_iter().map(to_dto));
    }

    Ok(Json(out))
}

fn to_dto(content: ControlledContent) -> ControlledDto {
    ControlledDto {
        alias: content.alias,
        content_type: match content.content_type {
            ControlledContentType::Artist => "artist",
            ControlledContentType::Character => "character",
            ControlledContentType::Tag => "tag",
        }
        .to_owned(),
        status: match content.control_method {
            ControlMethod::Banned => "banned",
            ControlMethod::Spoilered => "spoilered",
        }
        .to_owned(),
        reason: content.reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::Request,
    };
    use domain::ControlledContentId;
    use persistence::in_memory::InMemoryControlledContentRepository;
    use tower::ServiceExt; // for `oneshot`

    async fn seeded_app() -> Router {
        let repo = InMemoryControlledContentRepository::new();
        repo.control(
            "Zaush".into(),
            ControlledContentType::Artist,
            ControlMethod::Banned,
            Some("cub".into()),
        )
        .await
        .unwrap();
        repo.control(
            "Vore".into(),
            ControlledContentType::Tag,
            ControlMethod::Spoilered,
            None,
        )
        .await
        .unwrap();
        build_app(Arc::new(repo))
    }

    async fn get(app: Router, uri: &str) -> (StatusCode, serde_json::Value) {
        let response = app
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json = if bytes.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
        };
        (status, json)
    }

    #[tokio::test]
    async fn lists_all_types() {
        let (status, json) = get(seeded_app().await, "/api/controlled-content").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json.as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn filters_by_type_and_shapes_payload() {
        let (status, json) = get(seeded_app().await, "/api/controlled-content?type=artist").await;
        assert_eq!(status, StatusCode::OK);
        let arr = json.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["alias"], "Zaush");
        assert_eq!(arr[0]["type"], "artist");
        assert_eq!(arr[0]["status"], "banned");
        assert_eq!(arr[0]["reason"], "cub");
    }

    #[tokio::test]
    async fn null_reason_serializes_as_null() {
        let (_, json) = get(seeded_app().await, "/api/controlled-content?type=tag").await;
        assert_eq!(json[0]["status"], "spoilered");
        assert!(json[0]["reason"].is_null());
    }

    #[tokio::test]
    async fn unknown_type_is_bad_request() {
        let (status, _) = get(seeded_app().await, "/api/controlled-content?type=bogus").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn empty_repo_returns_empty_array() {
        let app = build_app(Arc::new(InMemoryControlledContentRepository::new()));
        let (status, json) = get(app, "/api/controlled-content").await;
        assert_eq!(status, StatusCode::OK);
        assert!(json.as_array().unwrap().is_empty());
    }

    #[test]
    fn to_dto_maps_enums_to_strings() {
        let content = ControlledContent::new(
            ControlledContentId::new(1),
            "x".into(),
            ControlledContentType::Character,
            ControlMethod::Spoilered,
            None,
        );
        let dto = to_dto(content);
        assert_eq!(dto.content_type, "character");
        assert_eq!(dto.status, "spoilered");
        assert!(dto.reason.is_none());
    }
}
