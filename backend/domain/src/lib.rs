use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledContentType {
    Artist,
    Character,
    Tag,
}

#[derive(Debug, Clone, Copy)]
pub enum ControlMethod {
    Banned,
    Spoilered,
}

pub type DomainDateTime = DateTime<Utc>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControlledContentId(u64);
impl ControlledContentId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(u64);
impl UserId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct ControlledContent {
    pub id: ControlledContentId,
    /// The alias of whatever is banned. The alias is assumed to be unique.
    pub alias: String,
    pub content_type: ControlledContentType,
    pub control_method: ControlMethod,
    pub reason: Option<String>,
    pub created_at: DomainDateTime,
    pub updated_at: DomainDateTime,
    pub deleted_at: Option<DomainDateTime>,
}

impl ControlledContent {
    pub fn new(
        id: ControlledContentId,
        alias: String,
        content_type: ControlledContentType,
        control_method: ControlMethod,
        reason: Option<String>,
    ) -> Self {
        let current_timestamp = chrono::Utc::now();

        Self {
            id,
            alias,
            content_type,
            control_method,
            reason,
            created_at: current_timestamp,
            updated_at: current_timestamp,
            deleted_at: None,
        }
    }

    /// Replace the reason and bump the `updated_at` timestamp.
    pub fn set_reason(&mut self, reason: Option<String>) {
        self.reason = reason;
        self.updated_at = chrono::Utc::now();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChangeEvent {
    Updated,
    Added,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChangelogEventId(u64);

impl ChangelogEventId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

/// Changelog for CONTROLLED CONTENT ONLY. We don't care about everything else. Github handles
/// diffs.
#[derive(Debug, Clone)]
pub struct ChangelogEvent {
    pub id: ChangelogEventId,
    pub event_type: ChangeEvent,
    pub target_id: ControlledContentId,
    pub by_user: UserId,
    pub at: DomainDateTime,
}

impl ChangelogEvent {
    /// Build a new event, stamping `at` with the current time.
    pub fn new(
        id: ChangelogEventId,
        event_type: ChangeEvent,
        target_id: ControlledContentId,
        by_user: UserId,
    ) -> Self {
        Self {
            id,
            event_type,
            target_id,
            by_user,
            at: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("This value already exists: {0}")]
    Conflict(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal Error")]
    InternalError,
}

#[async_trait::async_trait]
pub trait ControlledContentRepository {
    /// Start controlling a specific content alias
    async fn control(
        &self,
        alias: String,
        content_type: ControlledContentType,
        control_method: ControlMethod,
        reason: Option<String>,
    ) -> Result<ControlledContent, RepositoryError>;

    /// Stop controlling a specific content alias
    async fn deschedule(&self, alias: String) -> Result<(), RepositoryError>;

    /// Check if a specific alias is controlled
    async fn find_controlled(
        &self,
        alias: String,
    ) -> Result<Option<ControlledContent>, RepositoryError>;

    /// Add a reason as to why a specific alias is controlled
    async fn add_reason(
        &self,
        alias: String,
        reason: Option<String>,
    ) -> Result<ControlledContent, RepositoryError>;

    /// Find all the controlled members of provided content type
    async fn find_all_controlled_by(
        &self,
        content_type: ControlledContentType,
    ) -> Result<Vec<ControlledContent>, RepositoryError>;
}

#[async_trait::async_trait]
pub trait ChangelogRepository {
    async fn emit(
        &self,
        event: ChangeEvent,
        for_target: ControlledContentId,
        by_user: UserId,
    ) -> Result<ChangelogEvent, RepositoryError>;
    /// Fetch the changelog for a specific controlled-content target. Alias resolution happens at
    /// the service layer, so the repository deals purely in [`ControlledContentId`].
    async fn get_history(
        &self,
        for_target: ControlledContentId,
    ) -> Result<Vec<ChangelogEvent>, RepositoryError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread::sleep, time::Duration};

    #[test]
    fn id_wrappers_roundtrip() {
        assert_eq!(ControlledContentId::new(7).get(), 7);
        assert_eq!(UserId::new(42).get(), 42);
        assert_eq!(ChangelogEventId::new(99).get(), 99);
    }

    #[test]
    fn controlled_content_new_initializes_timestamps() {
        let content = ControlledContent::new(
            ControlledContentId::new(1),
            "Zaush".to_owned(),
            ControlledContentType::Artist,
            ControlMethod::Banned,
            Some("reason".to_owned()),
        );

        assert_eq!(content.alias, "Zaush");
        assert_eq!(content.content_type, ControlledContentType::Artist);
        assert!(matches!(content.control_method, ControlMethod::Banned));
        assert_eq!(content.reason.as_deref(), Some("reason"));
        // Freshly created: both timestamps equal, not yet deleted.
        assert_eq!(content.created_at, content.updated_at);
        assert!(content.deleted_at.is_none());
    }

    #[test]
    fn set_reason_replaces_value_and_bumps_updated_at() {
        let mut content = ControlledContent::new(
            ControlledContentId::new(1),
            "GearFox".to_owned(),
            ControlledContentType::Character,
            ControlMethod::Spoilered,
            None,
        );
        let created = content.created_at;

        sleep(Duration::from_millis(5));
        content.set_reason(Some("cub art".to_owned()));

        assert_eq!(content.reason.as_deref(), Some("cub art"));
        assert!(content.updated_at > created, "updated_at should advance");
        assert_eq!(content.created_at, created, "created_at must not change");
    }

    #[test]
    fn set_reason_can_clear() {
        let mut content = ControlledContent::new(
            ControlledContentId::new(1),
            "x".to_owned(),
            ControlledContentType::Tag,
            ControlMethod::Banned,
            Some("temp".to_owned()),
        );
        content.set_reason(None);
        assert!(content.reason.is_none());
    }

    #[test]
    fn changelog_event_new_sets_fields() {
        let event = ChangelogEvent::new(
            ChangelogEventId::new(3),
            ChangeEvent::Added,
            ControlledContentId::new(5),
            UserId::new(8),
        );

        assert_eq!(event.id.get(), 3);
        assert!(matches!(event.event_type, ChangeEvent::Added));
        assert_eq!(event.target_id, ControlledContentId::new(5));
        assert_eq!(event.by_user, UserId::new(8));
    }

    #[test]
    fn repository_error_messages() {
        assert_eq!(
            RepositoryError::Conflict("dup".to_owned()).to_string(),
            "This value already exists: dup"
        );
        assert_eq!(
            RepositoryError::NotFound("gone".to_owned()).to_string(),
            "Not found: gone"
        );
        assert_eq!(RepositoryError::InternalError.to_string(), "Internal Error");
    }
}
