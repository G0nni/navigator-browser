use super::value_objects::{TabId, ValidatedUrl, Certificate};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a browser tab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: TabId,
    pub title: String,
    pub url: Option<ValidatedUrl>,
    pub is_loading: bool,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub favicon_url: Option<String>,
}

impl Tab {
    pub fn new(is_private: bool) -> Self {
        let now = Utc::now();
        Self {
            id: TabId::new(),
            title: "New Tab".to_string(),
            url: None,
            is_loading: false,
            is_private,
            created_at: now,
            last_accessed: now,
            favicon_url: None,
        }
    }

    pub fn with_url(url: ValidatedUrl, is_private: bool) -> Self {
        let mut tab = Self::new(is_private);
        tab.url = Some(url);
        tab.is_loading = true;
        tab
    }

    pub fn update_url(&mut self, url: ValidatedUrl) {
        self.url = Some(url);
        self.last_accessed = Utc::now();
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }
}

/// Represents a bookmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: i64,
    pub title: String,
    pub url: ValidatedUrl,
    pub folder: Option<String>,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

impl Bookmark {
    pub fn new(title: String, url: ValidatedUrl) -> Self {
        Self {
            id: 0, // Will be set by database
            title,
            url,
            folder: None,
            created_at: Utc::now(),
            tags: Vec::new(),
        }
    }
}

/// Represents a history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub url: ValidatedUrl,
    pub title: String,
    pub visited_at: DateTime<Utc>,
    pub visit_count: i32,
}

impl HistoryEntry {
    pub fn new(url: ValidatedUrl, title: String) -> Self {
        Self {
            id: 0, // Will be set by database
            url,
            title,
            visited_at: Utc::now(),
            visit_count: 1,
        }
    }
}

/// Security context for a tab
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub is_secure: bool,
    pub certificate: Option<Certificate>,
    pub has_mixed_content: bool,
    pub permissions: Vec<Permission>,
}

impl SecurityContext {
    pub fn new() -> Self {
        Self {
            is_secure: false,
            certificate: None,
            has_mixed_content: false,
            permissions: Vec::new(),
        }
    }

    pub fn with_https(certificate: Certificate) -> Self {
        Self {
            is_secure: true,
            certificate: Some(certificate),
            has_mixed_content: false,
            permissions: Vec::new(),
        }
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Permissions that can be requested by websites
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    Camera,
    Microphone,
    Location,
    Notifications,
    Storage,
}
