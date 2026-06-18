use std::{
    collections::HashMap,
    sync::{
        RwLock,
        atomic::{AtomicU64, Ordering},
    },
};

use domain::{
    ChangeEvent, ChangelogEvent, ChangelogEventId, ChangelogRepository, ControlMethod,
    ControlledContent, ControlledContentId, ControlledContentRepository, ControlledContentType,
    RepositoryError, UserId,
};

pub struct InMemoryControlledContentRepository {
    pub controlled_content: RwLock<HashMap<ControlledContentId, ControlledContent>>,
    next_id: AtomicU64,
}

impl InMemoryControlledContentRepository {
    pub fn new() -> Self {
        Self {
            controlled_content: RwLock::new(HashMap::new()),
            next_id: AtomicU64::new(0),
        }
    }

    fn next_id(&self) -> ControlledContentId {
        ControlledContentId::new(self.next_id.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for InMemoryControlledContentRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ControlledContentRepository for InMemoryControlledContentRepository {
    async fn control(
        &self,
        alias: String,
        content_type: ControlledContentType,
        control_method: ControlMethod,
        reason: Option<String>,
    ) -> Result<ControlledContent, RepositoryError> {
        let mut controlled_content = self
            .controlled_content
            .write()
            .map_err(|_| RepositoryError::InternalError)?;

        if controlled_content
            .values()
            .any(|content| content.alias == alias)
        {
            return Err(RepositoryError::Conflict(alias));
        }

        let id = self.next_id();
        let new_control = ControlledContent::new(id, alias, content_type, control_method, reason);
        controlled_content.insert(id, new_control.clone());

        Ok(new_control)
    }

    async fn deschedule(&self, alias: String) -> Result<(), RepositoryError> {
        let mut controlled_content = self
            .controlled_content
            .write()
            .map_err(|_| RepositoryError::InternalError)?;

        controlled_content.retain(|_, content| content.alias != alias);

        Ok(())
    }

    async fn find_controlled(
        &self,
        alias: String,
    ) -> Result<Option<ControlledContent>, RepositoryError> {
        let controlled_content = self
            .controlled_content
            .read()
            .map_err(|_| RepositoryError::InternalError)?;

        Ok(controlled_content
            .values()
            .find(|content| content.alias == alias)
            .cloned())
    }

    async fn add_reason(
        &self,
        alias: String,
        reason: Option<String>,
    ) -> Result<ControlledContent, RepositoryError> {
        let mut controlled_content = self
            .controlled_content
            .write()
            .map_err(|_| RepositoryError::InternalError)?;

        match controlled_content
            .values_mut()
            .find(|content| content.alias == alias)
        {
            Some(content) => {
                content.set_reason(reason);
                Ok(content.clone())
            }
            None => Err(RepositoryError::NotFound(alias)),
        }
    }

    async fn find_all_controlled_by(
        &self,
        content_type: ControlledContentType,
    ) -> Result<Vec<ControlledContent>, RepositoryError> {
        let controlled_content = self
            .controlled_content
            .read()
            .map_err(|_| RepositoryError::InternalError)?;

        Ok(controlled_content
            .values()
            .filter(|content| content.content_type == content_type)
            .cloned()
            .collect())
    }
}

pub struct InMemoryChangelogRepository {
    pub events: RwLock<HashMap<ChangelogEventId, ChangelogEvent>>,
    next_id: AtomicU64,
}

impl InMemoryChangelogRepository {
    pub fn new() -> Self {
        Self {
            events: RwLock::new(HashMap::new()),
            next_id: AtomicU64::new(0),
        }
    }

    fn next_id(&self) -> ChangelogEventId {
        ChangelogEventId::new(self.next_id.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for InMemoryChangelogRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ChangelogRepository for InMemoryChangelogRepository {
    async fn emit(
        &self,
        event: ChangeEvent,
        for_target: ControlledContentId,
        by_user: UserId,
    ) -> Result<ChangelogEvent, RepositoryError> {
        let mut events = self
            .events
            .write()
            .map_err(|_| RepositoryError::InternalError)?;

        let id = self.next_id();
        let new_event = ChangelogEvent::new(id, event, for_target, by_user);
        events.insert(id, new_event.clone());

        Ok(new_event)
    }

    async fn get_history(
        &self,
        for_target: ControlledContentId,
    ) -> Result<Vec<ChangelogEvent>, RepositoryError> {
        let events = self
            .events
            .read()
            .map_err(|_| RepositoryError::InternalError)?;

        let mut history: Vec<ChangelogEvent> = events
            .values()
            .filter(|event| event.target_id == for_target)
            .cloned()
            .collect();

        history.sort_by_key(|event| event.at);

        Ok(history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{ChangeEvent, ChangelogRepository, UserId};

    #[tokio::test]
    async fn control_then_find() {
        let repo = InMemoryControlledContentRepository::new();
        let created = repo
            .control(
                "Zaush".into(),
                ControlledContentType::Artist,
                ControlMethod::Banned,
                Some("reason".into()),
            )
            .await
            .unwrap();

        let found = repo.find_controlled("Zaush".into()).await.unwrap().unwrap();
        assert_eq!(found.id, created.id);
        assert_eq!(found.reason.as_deref(), Some("reason"));
        assert!(repo.find_controlled("nope".into()).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn duplicate_alias_conflicts() {
        let repo = InMemoryControlledContentRepository::new();
        repo.control(
            "dup".into(),
            ControlledContentType::Tag,
            ControlMethod::Banned,
            None,
        )
        .await
        .unwrap();
        let err = repo
            .control(
                "dup".into(),
                ControlledContentType::Tag,
                ControlMethod::Spoilered,
                None,
            )
            .await
            .unwrap_err();
        assert!(matches!(err, RepositoryError::Conflict(a) if a == "dup"));
    }

    #[tokio::test]
    async fn ids_increment() {
        let repo = InMemoryControlledContentRepository::new();
        let a = repo
            .control(
                "a".into(),
                ControlledContentType::Artist,
                ControlMethod::Banned,
                None,
            )
            .await
            .unwrap();
        let b = repo
            .control(
                "b".into(),
                ControlledContentType::Artist,
                ControlMethod::Banned,
                None,
            )
            .await
            .unwrap();
        assert_ne!(a.id, b.id);
    }

    #[tokio::test]
    async fn deschedule_removes() {
        let repo = InMemoryControlledContentRepository::new();
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
        // Descheduling a missing alias is a no-op, not an error.
        repo.deschedule("ghost".into()).await.unwrap();
    }

    #[tokio::test]
    async fn add_reason_success_and_not_found() {
        let repo = InMemoryControlledContentRepository::new();
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

        let err = repo.add_reason("missing".into(), None).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(a) if a == "missing"));
    }

    #[tokio::test]
    async fn find_all_by_type_filters() {
        let repo = InMemoryControlledContentRepository::new();
        repo.control(
            "art".into(),
            ControlledContentType::Artist,
            ControlMethod::Banned,
            None,
        )
        .await
        .unwrap();
        repo.control(
            "tg".into(),
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
        assert_eq!(artists[0].alias, "art");

        let chars = repo
            .find_all_controlled_by(ControlledContentType::Character)
            .await
            .unwrap();
        assert!(chars.is_empty());
    }

    #[tokio::test]
    async fn changelog_emit_and_history() {
        let repo = InMemoryChangelogRepository::new();
        let target = ControlledContentId::new(1);

        repo.emit(ChangeEvent::Added, target, UserId::new(10))
            .await
            .unwrap();
        repo.emit(ChangeEvent::Updated, target, UserId::new(11))
            .await
            .unwrap();
        // Different target should not leak into this history.
        repo.emit(
            ChangeEvent::Added,
            ControlledContentId::new(2),
            UserId::new(12),
        )
        .await
        .unwrap();

        let history = repo.get_history(target).await.unwrap();
        assert_eq!(history.len(), 2);
        assert!(history.iter().all(|e| e.target_id == target));

        assert!(
            repo.get_history(ControlledContentId::new(999))
                .await
                .unwrap()
                .is_empty()
        );
    }
}
