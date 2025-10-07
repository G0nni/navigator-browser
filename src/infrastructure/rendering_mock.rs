// Mock rendering engine for Windows (sans WebKit)
use crate::domain::{RenderingEngine, ValidatedUrl};
use anyhow::Result;
use async_trait::async_trait;

/// Mock rendering engine pour tests sur Windows
pub struct MockRenderer {
    current_url: std::sync::RwLock<Option<ValidatedUrl>>,
    current_title: std::sync::RwLock<String>,
}

impl MockRenderer {
    pub fn new() -> Self {
        Self {
            current_url: std::sync::RwLock::new(None),
            current_title: std::sync::RwLock::new("New Tab".to_string()),
        }
    }
}

impl Default for MockRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RenderingEngine for MockRenderer {
    async fn load_url(&self, url: &ValidatedUrl) -> Result<()> {
        tracing::info!("Mock: Loading URL: {}", url);

        if let Ok(mut current) = self.current_url.write() {
            *current = Some(url.clone());
        }

        // Simuler le titre depuis l'URL
        if let Some(host) = url.host_str() {
            if let Ok(mut title) = self.current_title.write() {
                *title = host.to_string();
            }
        }

        Ok(())
    }

    async fn get_title(&self) -> Result<String> {
        if let Ok(title) = self.current_title.read() {
            Ok(title.clone())
        } else {
            Ok("Untitled".to_string())
        }
    }

    async fn execute_javascript(&self, script: &str) -> Result<String> {
        tracing::debug!("Mock: JavaScript execution requested: {}", script);
        Ok(String::new())
    }

    async fn take_screenshot(&self) -> Result<Vec<u8>> {
        tracing::debug!("Mock: Screenshot requested");
        Ok(Vec::new())
    }
}
