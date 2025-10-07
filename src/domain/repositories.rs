use super::entities::{Bookmark, HistoryEntry, Tab};
use super::value_objects::{TabId, ValidatedUrl};
use async_trait::async_trait;
use anyhow::Result;

/// Repository for managing tabs persistence
#[async_trait]
pub trait TabRepository: Send + Sync {
    async fn save(&self, tab: &Tab) -> Result<()>;
    async fn find_by_id(&self, id: TabId) -> Result<Option<Tab>>;
    async fn find_all(&self) -> Result<Vec<Tab>>;
    async fn delete(&self, id: TabId) -> Result<()>;
    async fn save_session(&self, tabs: Vec<Tab>) -> Result<()>;
    async fn restore_session(&self) -> Result<Vec<Tab>>;
}

/// Repository for managing bookmarks
#[async_trait]
pub trait BookmarkRepository: Send + Sync {
    async fn save(&self, bookmark: &Bookmark) -> Result<i64>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Bookmark>>;
    async fn find_all(&self) -> Result<Vec<Bookmark>>;
    async fn find_by_folder(&self, folder: &str) -> Result<Vec<Bookmark>>;
    async fn search(&self, query: &str) -> Result<Vec<Bookmark>>;
    async fn delete(&self, id: i64) -> Result<()>;
    async fn update(&self, bookmark: &Bookmark) -> Result<()>;
}

/// Repository for managing browsing history
#[async_trait]
pub trait HistoryRepository: Send + Sync {
    async fn add(&self, entry: &HistoryEntry) -> Result<i64>;
    async fn find_by_url(&self, url: &ValidatedUrl) -> Result<Option<HistoryEntry>>;
    async fn search(&self, query: &str, limit: i32) -> Result<Vec<HistoryEntry>>;
    async fn get_recent(&self, limit: i32) -> Result<Vec<HistoryEntry>>;
    async fn delete_by_url(&self, url: &ValidatedUrl) -> Result<()>;
    async fn clear_all(&self) -> Result<()>;
    async fn increment_visit_count(&self, url: &ValidatedUrl) -> Result<()>;
}
