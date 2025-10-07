use crate::domain::{
    Bookmark, BookmarkRepository, HistoryEntry, HistoryRepository, RenderingEngine,
    SecurityService, Tab, TabId, TabRepository, ValidatedUrl,
};
use anyhow::{anyhow, Context, Result};
use std::sync::Arc;

use super::state::BrowserState;

/// Use case: Open a new tab
pub struct OpenTabUseCase {
    state: BrowserState,
    tab_repository: Arc<dyn TabRepository>,
}

impl OpenTabUseCase {
    pub fn new(state: BrowserState, tab_repository: Arc<dyn TabRepository>) -> Self {
        Self {
            state,
            tab_repository,
        }
    }

    pub async fn execute(&self, url: Option<ValidatedUrl>) -> Result<TabId> {
        let is_private = self.state.is_private_mode();

        let tab = match url {
            Some(url) => Tab::with_url(url, is_private),
            None => Tab::new(is_private),
        };

        let tab_id = tab.id;

        // Save to repository if not in private mode
        if !is_private {
            self.tab_repository.save(&tab).await?;
        }

        // Add to state
        self.state.add_tab(tab);
        self.state.set_active_tab(tab_id);

        tracing::info!("Opened new tab: {}", tab_id);

        Ok(tab_id)
    }
}

/// Use case: Close a tab
pub struct CloseTabUseCase {
    state: BrowserState,
    tab_repository: Arc<dyn TabRepository>,
}

impl CloseTabUseCase {
    pub fn new(state: BrowserState, tab_repository: Arc<dyn TabRepository>) -> Self {
        Self {
            state,
            tab_repository,
        }
    }

    pub async fn execute(&self, tab_id: TabId) -> Result<()> {
        // Remove from state
        let tab = self
            .state
            .remove_tab(tab_id)
            .ok_or_else(|| anyhow!("Tab not found"))?;

        // Delete from repository if not private
        if !tab.is_private {
            self.tab_repository.delete(tab_id).await?;
        }

        // If this was the active tab, activate another
        if self.state.get_active_tab_id() == Some(tab_id) {
            let tabs = self.state.get_all_tabs();
            if let Some(next_tab) = tabs.first() {
                self.state.set_active_tab(next_tab.id);
            }
        }

        tracing::info!("Closed tab: {}", tab_id);

        Ok(())
    }
}

/// Use case: Navigate to a URL
pub struct NavigateUseCase {
    state: BrowserState,
    security_service: Arc<dyn SecurityService>,
    history_repository: Arc<dyn HistoryRepository>,
    rendering_engine: Arc<dyn RenderingEngine>,
}

impl NavigateUseCase {
    pub fn new(
        state: BrowserState,
        security_service: Arc<dyn SecurityService>,
        history_repository: Arc<dyn HistoryRepository>,
        rendering_engine: Arc<dyn RenderingEngine>,
    ) -> Self {
        Self {
            state,
            security_service,
            history_repository,
            rendering_engine,
        }
    }

    pub async fn execute(&self, tab_id: TabId, url_str: &str) -> Result<()> {
        // Validate URL
        let url = self
            .security_service
            .validate_url(url_str)
            .context("Invalid URL")?;

        // Check if URL is blocked
        if self.security_service.is_blocked(&url) {
            return Err(anyhow!("This URL is blocked for security reasons"));
        }

        // Get the tab
        let mut tab = self
            .state
            .get_tab(tab_id)
            .ok_or_else(|| anyhow!("Tab not found"))?;

        // Update tab state
        tab.update_url(url.clone());
        tab.set_loading(true);
        self.state.update_tab(tab.clone());

        tracing::info!("Navigating tab {} to {}", tab_id, url);

        // Load URL in rendering engine
        self.rendering_engine
            .load_url(&url)
            .await
            .context("Failed to load URL")?;

        // Add to history if not in private mode
        if !tab.is_private {
            let title = self
                .rendering_engine
                .get_title()
                .await
                .unwrap_or_else(|_| url.as_str().to_string());

            let entry = HistoryEntry::new(url.clone(), title.clone());
            self.history_repository.add(&entry).await?;

            // Update tab title
            tab.update_title(title);
        }

        // Mark as loaded
        tab.set_loading(false);
        self.state.update_tab(tab);

        Ok(())
    }
}

/// Use case: Save a bookmark
pub struct SaveBookmarkUseCase {
    bookmark_repository: Arc<dyn BookmarkRepository>,
}

impl SaveBookmarkUseCase {
    pub fn new(bookmark_repository: Arc<dyn BookmarkRepository>) -> Self {
        Self {
            bookmark_repository,
        }
    }

    pub async fn execute(&self, title: String, url: ValidatedUrl) -> Result<i64> {
        let bookmark = Bookmark::new(title, url);
        let id = self.bookmark_repository.save(&bookmark).await?;

        tracing::info!("Saved bookmark: {}", id);

        Ok(id)
    }
}

/// Use case: Search history
pub struct SearchHistoryUseCase {
    history_repository: Arc<dyn HistoryRepository>,
}

impl SearchHistoryUseCase {
    pub fn new(history_repository: Arc<dyn HistoryRepository>) -> Self {
        Self { history_repository }
    }

    pub async fn execute(&self, query: &str, limit: i32) -> Result<Vec<HistoryEntry>> {
        self.history_repository.search(query, limit).await
    }
}

/// Use case: Get recent history
pub struct GetRecentHistoryUseCase {
    history_repository: Arc<dyn HistoryRepository>,
}

impl GetRecentHistoryUseCase {
    pub fn new(history_repository: Arc<dyn HistoryRepository>) -> Self {
        Self { history_repository }
    }

    pub async fn execute(&self, limit: i32) -> Result<Vec<HistoryEntry>> {
        self.history_repository.get_recent(limit).await
    }
}

/// Use case: Clear browsing data
pub struct ClearBrowsingDataUseCase {
    history_repository: Arc<dyn HistoryRepository>,
}

impl ClearBrowsingDataUseCase {
    pub fn new(history_repository: Arc<dyn HistoryRepository>) -> Self {
        Self { history_repository }
    }

    pub async fn execute(&self) -> Result<()> {
        self.history_repository.clear_all().await?;
        tracing::info!("Cleared all browsing data");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::SqliteDatabase;

    #[tokio::test]
    async fn test_open_tab_use_case() {
        let state = BrowserState::new();
        let db = Arc::new(SqliteDatabase::new(":memory:").await.unwrap());

        let use_case = OpenTabUseCase::new(state.clone(), db);
        let tab_id = use_case.execute(None).await.unwrap();

        assert_eq!(state.tab_count(), 1);
        assert_eq!(state.get_active_tab_id(), Some(tab_id));
    }
}
