use crate::domain::{Tab, TabId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Manages the browser's runtime state
#[derive(Clone)]
pub struct BrowserState {
    tabs: Arc<RwLock<HashMap<TabId, Tab>>>,
    active_tab: Arc<RwLock<Option<TabId>>>,
    is_private_mode: Arc<RwLock<bool>>,
}

impl BrowserState {
    pub fn new() -> Self {
        Self {
            tabs: Arc::new(RwLock::new(HashMap::new())),
            active_tab: Arc::new(RwLock::new(None)),
            is_private_mode: Arc::new(RwLock::new(false)),
        }
    }

    /// Add a new tab
    pub fn add_tab(&self, tab: Tab) -> TabId {
        let tab_id = tab.id;
        if let Ok(mut tabs) = self.tabs.write() {
            tabs.insert(tab_id, tab);
        }
        tab_id
    }

    /// Remove a tab
    pub fn remove_tab(&self, tab_id: TabId) -> Option<Tab> {
        if let Ok(mut tabs) = self.tabs.write() {
            return tabs.remove(&tab_id);
        }
        None
    }

    /// Get a tab by ID
    pub fn get_tab(&self, tab_id: TabId) -> Option<Tab> {
        if let Ok(tabs) = self.tabs.read() {
            return tabs.get(&tab_id).cloned();
        }
        None
    }

    /// Update a tab
    pub fn update_tab(&self, tab: Tab) {
        if let Ok(mut tabs) = self.tabs.write() {
            tabs.insert(tab.id, tab);
        }
    }

    /// Get all tabs
    pub fn get_all_tabs(&self) -> Vec<Tab> {
        if let Ok(tabs) = self.tabs.read() {
            return tabs.values().cloned().collect();
        }
        Vec::new()
    }

    /// Get the number of tabs
    pub fn tab_count(&self) -> usize {
        if let Ok(tabs) = self.tabs.read() {
            return tabs.len();
        }
        0
    }

    /// Set the active tab
    pub fn set_active_tab(&self, tab_id: TabId) {
        if let Ok(mut active) = self.active_tab.write() {
            *active = Some(tab_id);
        }
    }

    /// Get the active tab ID
    pub fn get_active_tab_id(&self) -> Option<TabId> {
        if let Ok(active) = self.active_tab.read() {
            return *active;
        }
        None
    }

    /// Get the active tab
    pub fn get_active_tab(&self) -> Option<Tab> {
        if let Some(tab_id) = self.get_active_tab_id() {
            return self.get_tab(tab_id);
        }
        None
    }

    /// Set private mode
    pub fn set_private_mode(&self, enabled: bool) {
        if let Ok(mut mode) = self.is_private_mode.write() {
            *mode = enabled;
        }
    }

    /// Check if in private mode
    pub fn is_private_mode(&self) -> bool {
        if let Ok(mode) = self.is_private_mode.read() {
            return *mode;
        }
        false
    }

    /// Clear all tabs
    pub fn clear_all_tabs(&self) {
        if let Ok(mut tabs) = self.tabs.write() {
            tabs.clear();
        }
        if let Ok(mut active) = self.active_tab.write() {
            *active = None;
        }
    }
}

impl Default for BrowserState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ValidatedUrl;

    #[test]
    fn test_add_and_get_tab() {
        let state = BrowserState::new();
        let tab = Tab::new(false);
        let tab_id = tab.id;

        state.add_tab(tab);

        let retrieved = state.get_tab(tab_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, tab_id);
    }

    #[test]
    fn test_remove_tab() {
        let state = BrowserState::new();
        let tab = Tab::new(false);
        let tab_id = tab.id;

        state.add_tab(tab);
        assert_eq!(state.tab_count(), 1);

        state.remove_tab(tab_id);
        assert_eq!(state.tab_count(), 0);
    }

    #[test]
    fn test_active_tab() {
        let state = BrowserState::new();
        let tab = Tab::new(false);
        let tab_id = tab.id;

        state.add_tab(tab);
        state.set_active_tab(tab_id);

        assert_eq!(state.get_active_tab_id(), Some(tab_id));
    }

    #[test]
    fn test_private_mode() {
        let state = BrowserState::new();
        assert!(!state.is_private_mode());

        state.set_private_mode(true);
        assert!(state.is_private_mode());
    }
}
