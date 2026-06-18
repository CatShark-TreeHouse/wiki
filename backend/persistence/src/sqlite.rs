use chrono::{DateTime, Utc};
use domain::{
    ChangeEvent, ChangelogEvent, ChangelogEventId, ChangelogRepository, ControlMethod,
    ControlledContent, ControlledContentId, ControlledContentRepository, ControlledContentType,
    RepositoryError, UserId,
};
use sqlx::{Row, SqlitePool, sqlite::SqliteRow};

/// Create the schema if it does not yet exist. Safe to call on every startup.
pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS controlled_content (
            id             INTEGER PRIMARY KEY AUTOINCREMENT,
            alias          TEXT NOT NULL UNIQUE,
            content_type   TEXT NOT NULL,
            control_method TEXT NOT NULL,
            reason         TEXT,
            created_at     TEXT NOT NULL,
            updated_at     TEXT NOT NULL,
            deleted_at     TEXT
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS changelog (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            event_type TEXT NOT NULL,
            target_id  INTEGER NOT NULL,
            by_user    INTEGER NOT NULL,
            at         TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ---- enum <-> text helpers -------------------------------------------------

fn content_type_to_str(content_type: ControlledContentType) -> &'static str {
    match content_type {
        ControlledContentType::Artist => "artist",
        ControlledContentType::Character => "character",
        ControlledContentType::Tag => "tag",
    }
}

fn content_type_from_str(value: &str) -> Result<ControlledContentType, RepositoryError> {
    match value {
        "artist" => Ok(ControlledContentType::Artist),
        "character" => Ok(ControlledContentType::Character),
        "tag" => Ok(ControlledContentType::Tag),
        _ => Err(RepositoryError::InternalError),
    }
}

fn control_method_to_str(method: ControlMethod) -> &'static str {
    match method {
        ControlMethod::Banned => "banned",
        ControlMethod::Spoilered => "spoilered",
    }
}

fn control_method_from_str(value: &str) -> Result<ControlMethod, RepositoryError> {
    match value {
        "banned" => Ok(ControlMethod::Banned),
        "spoilered" => Ok(ControlMethod::Spoilered),
        _ => Err(RepositoryError::InternalError),
    }
}

fn change_event_to_str(event: ChangeEvent) -> &'static str {
    match event {
        ChangeEvent::Updated => "updated",
        ChangeEvent::Added => "added",
        ChangeEvent::Deleted => "deleted",
    }
}

fn change_event_from_str(value: &str) -> Result<ChangeEvent, RepositoryError> {
    match value {
        "updated" => Ok(ChangeEvent::Updated),
        "added" => Ok(ChangeEvent::Added),
        "deleted" => Ok(ChangeEvent::Deleted),
        _ => Err(RepositoryError::InternalError),
    }
}

fn internal<E>(_: E) -> RepositoryError {
    RepositoryError::InternalError
}

fn row_to_content(row: &SqliteRow) -> Result<ControlledContent, RepositoryError> {
    let id: i64 = row.try_get("id").map_err(internal)?;
    let content_type: String = row.try_get("content_type").map_err(internal)?;
    let control_method: String = row.try_get("control_method").map_err(internal)?;
    Ok(ControlledContent {
        id: ControlledContentId::new(id as u64),
        alias: row.try_get("alias").map_err(internal)?,
        content_type: content_type_from_str(&content_type)?,
        control_method: control_method_from_str(&control_method)?,
        reason: row.try_get("reason").map_err(internal)?,
        created_at: row.try_get("created_at").map_err(internal)?,
        updated_at: row.try_get("updated_at").map_err(internal)?,
        deleted_at: row.try_get("deleted_at").map_err(internal)?,
    })
}

fn row_to_event(row: &SqliteRow) -> Result<ChangelogEvent, RepositoryError> {
    let id: i64 = row.try_get("id").map_err(internal)?;
    let target_id: i64 = row.try_get("target_id").map_err(internal)?;
    let by_user: i64 = row.try_get("by_user").map_err(internal)?;
    let event_type: String = row.try_get("event_type").map_err(internal)?;
    Ok(ChangelogEvent {
        id: ChangelogEventId::new(id as u64),
        event_type: change_event_from_str(&event_type)?,
        target_id: ControlledContentId::new(target_id as u64),
        by_user: UserId::new(by_user as u64),
        at: row.try_get("at").map_err(internal)?,
    })
}

// ---- ControlledContent repository ------------------------------------------

pub struct SqliteControlledContentRepository {
    pool: SqlitePool,
}

impl SqliteControlledContentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ControlledContentRepository for SqliteControlledContentRepository {
    async fn control(
        &self,
        alias: String,
        content_type: ControlledContentType,
        control_method: ControlMethod,
        reason: Option<String>,
    ) -> Result<ControlledContent, RepositoryError> {
        let now: DateTime<Utc> = Utc::now();

        let result = sqlx::query(
            "INSERT INTO controlled_content
                (alias, content_type, control_method, reason, created_at, updated_at, deleted_at)
             VALUES (?, ?, ?, ?, ?, ?, NULL)
             RETURNING id",
        )
        .bind(&alias)
        .bind(content_type_to_str(content_type))
        .bind(control_method_to_str(control_method))
        .bind(&reason)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await;

        let row = match result {
            Ok(row) => row,
            Err(sqlx::Error::Database(db)) if db.is_unique_violation() => {
                return Err(RepositoryError::Conflict(alias));
            }
            Err(_) => return Err(RepositoryError::InternalError),
        };

        let id: i64 = row.try_get("id").map_err(internal)?;

        Ok(ControlledContent {
            id: ControlledContentId::new(id as u64),
            alias,
            content_type,
            control_method,
            reason,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }

    async fn deschedule(&self, alias: String) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM controlled_content WHERE alias = ?")
            .bind(alias)
            .execute(&self.pool)
            .await
            .map_err(internal)?;
        Ok(())
    }

    async fn find_controlled(
        &self,
        alias: String,
    ) -> Result<Option<ControlledContent>, RepositoryError> {
        let row = sqlx::query("SELECT * FROM controlled_content WHERE alias = ?")
            .bind(alias)
            .fetch_optional(&self.pool)
            .await
            .map_err(internal)?;

        row.as_ref().map(row_to_content).transpose()
    }

    async fn add_reason(
        &self,
        alias: String,
        reason: Option<String>,
    ) -> Result<ControlledContent, RepositoryError> {
        let row = sqlx::query(
            "UPDATE controlled_content SET reason = ?, updated_at = ? WHERE alias = ? RETURNING *",
        )
        .bind(&reason)
        .bind(Utc::now())
        .bind(&alias)
        .fetch_optional(&self.pool)
        .await
        .map_err(internal)?;

        match row {
            Some(row) => row_to_content(&row),
            None => Err(RepositoryError::NotFound(alias)),
        }
    }

    async fn find_all_controlled_by(
        &self,
        content_type: ControlledContentType,
    ) -> Result<Vec<ControlledContent>, RepositoryError> {
        let rows = sqlx::query("SELECT * FROM controlled_content WHERE content_type = ?")
            .bind(content_type_to_str(content_type))
            .fetch_all(&self.pool)
            .await
            .map_err(internal)?;

        rows.iter().map(row_to_content).collect()
    }
}

// ---- Changelog repository --------------------------------------------------

pub struct SqliteChangelogRepository {
    pool: SqlitePool,
}

impl SqliteChangelogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ChangelogRepository for SqliteChangelogRepository {
    async fn emit(
        &self,
        event: ChangeEvent,
        for_target: ControlledContentId,
        by_user: UserId,
    ) -> Result<ChangelogEvent, RepositoryError> {
        let now: DateTime<Utc> = Utc::now();

        let row = sqlx::query(
            "INSERT INTO changelog (event_type, target_id, by_user, at)
             VALUES (?, ?, ?, ?)
             RETURNING id",
        )
        .bind(change_event_to_str(event))
        .bind(for_target.get() as i64)
        .bind(by_user.get() as i64)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(internal)?;

        let id: i64 = row.try_get("id").map_err(internal)?;

        Ok(ChangelogEvent {
            id: ChangelogEventId::new(id as u64),
            event_type: event,
            target_id: for_target,
            by_user,
            at: now,
        })
    }

    async fn get_history(
        &self,
        for_target: ControlledContentId,
    ) -> Result<Vec<ChangelogEvent>, RepositoryError> {
        let rows = sqlx::query("SELECT * FROM changelog WHERE target_id = ? ORDER BY at")
            .bind(for_target.get() as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(internal)?;

        rows.iter().map(row_to_event).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn memory_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        init_db(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn control_then_find_roundtrips() {
        let pool = memory_pool().await;
        let repo = SqliteControlledContentRepository::new(pool);

        let created = repo
            .control(
                "Zaush".into(),
                ControlledContentType::Artist,
                ControlMethod::Banned,
                Some("reason".into()),
            )
            .await
            .unwrap();
        assert_eq!(created.alias, "Zaush");

        let found = repo.find_controlled("Zaush".into()).await.unwrap().unwrap();
        assert_eq!(found.id, created.id);
        assert_eq!(found.content_type, ControlledContentType::Artist);
        assert!(matches!(found.control_method, ControlMethod::Banned));
        assert_eq!(found.reason.as_deref(), Some("reason"));

        assert!(
            repo.find_controlled("missing".into())
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn duplicate_alias_conflicts() {
        let pool = memory_pool().await;
        let repo = SqliteControlledContentRepository::new(pool);

        repo.control(
            "dup".into(),
            ControlledContentType::Tag,
            ControlMethod::Spoilered,
            None,
        )
        .await
        .unwrap();
        let err = repo
            .control(
                "dup".into(),
                ControlledContentType::Tag,
                ControlMethod::Banned,
                None,
            )
            .await
            .unwrap_err();
        assert!(matches!(err, RepositoryError::Conflict(alias) if alias == "dup"));
    }

    #[tokio::test]
    async fn find_all_by_type_filters() {
        let pool = memory_pool().await;
        let repo = SqliteControlledContentRepository::new(pool);

        repo.control(
            "a".into(),
            ControlledContentType::Artist,
            ControlMethod::Banned,
            None,
        )
        .await
        .unwrap();
        repo.control(
            "t".into(),
            ControlledContentType::Tag,
            ControlMethod::Banned,
            None,
        )
        .await
        .unwrap();

        let artists = repo
            .find_all_controlled_by(ControlledContentType::Artist)
            .await
            .unwrap();
        assert_eq!(artists.len(), 1);
        assert_eq!(artists[0].alias, "a");
    }

    #[tokio::test]
    async fn deschedule_removes_row() {
        let pool = memory_pool().await;
        let repo = SqliteControlledContentRepository::new(pool);

        repo.control(
            "x".into(),
            ControlledContentType::Tag,
            ControlMethod::Banned,
            None,
        )
        .await
        .unwrap();
        repo.deschedule("x".into()).await.unwrap();
        assert!(repo.find_controlled("x".into()).await.unwrap().is_none());
        // No-op for a missing alias.
        repo.deschedule("ghost".into()).await.unwrap();
    }

    #[tokio::test]
    async fn add_reason_success_and_not_found() {
        let pool = memory_pool().await;
        let repo = SqliteControlledContentRepository::new(pool);

        repo.control(
            "a".into(),
            ControlledContentType::Artist,
            ControlMethod::Banned,
            None,
        )
        .await
        .unwrap();

        let updated = repo
            .add_reason("a".into(), Some("because".into()))
            .await
            .unwrap();
        assert_eq!(updated.reason.as_deref(), Some("because"));
        // Re-reading confirms the update persisted.
        let found = repo.find_controlled("a".into()).await.unwrap().unwrap();
        assert_eq!(found.reason.as_deref(), Some("because"));

        let err = repo.add_reason("missing".into(), None).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(a) if a == "missing"));
    }

    #[tokio::test]
    async fn find_all_empty_for_unused_type() {
        let pool = memory_pool().await;
        let repo = SqliteControlledContentRepository::new(pool);
        let rows = repo
            .find_all_controlled_by(ControlledContentType::Character)
            .await
            .unwrap();
        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn changelog_emit_and_history() {
        let pool = memory_pool().await;
        let changelog = SqliteChangelogRepository::new(pool);
        let target = ControlledContentId::new(7);

        changelog
            .emit(ChangeEvent::Added, target, UserId::new(42))
            .await
            .unwrap();

        let history = changelog.get_history(target).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].target_id, target);
        assert_eq!(history[0].by_user, UserId::new(42));

        assert!(
            changelog
                .get_history(ControlledContentId::new(999))
                .await
                .unwrap()
                .is_empty()
        );
    }
}
