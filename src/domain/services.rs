use super::entities::SecurityContext;
use super::value_objects::{ValidatedUrl, Certificate};
use async_trait::async_trait;
use anyhow::Result;

/// Service for handling network requests securely
#[async_trait]
pub trait NetworkService: Send + Sync {
    async fn fetch(&self, url: &ValidatedUrl) -> Result<Vec<u8>>;
    async fn verify_certificate(&self, url: &ValidatedUrl) -> Result<Certificate>;
    async fn check_security(&self, url: &ValidatedUrl) -> Result<SecurityContext>;
}

/// Service for rendering web content
#[async_trait]
pub trait RenderingEngine: Send + Sync {
    async fn load_url(&self, url: &ValidatedUrl) -> Result<()>;
    async fn get_title(&self) -> Result<String>;
    async fn execute_javascript(&self, script: &str) -> Result<String>;
    async fn take_screenshot(&self) -> Result<Vec<u8>>;
}

/// Service for content security policy enforcement
pub trait SecurityService: Send + Sync {
    /// Validate if URL is safe to navigate to
    fn validate_url(&self, url: &str) -> Result<ValidatedUrl>;

    /// Check if URL should be blocked (malware, phishing, etc.)
    fn is_blocked(&self, url: &ValidatedUrl) -> bool;

    /// Sanitize HTML content to prevent XSS
    fn sanitize_html(&self, html: &str) -> String;

    /// Check if mixed content should be allowed
    fn allow_mixed_content(&self, url: &ValidatedUrl) -> bool;
}

/// Service for managing content blockers (ads, trackers)
#[async_trait]
pub trait ContentBlockerService: Send + Sync {
    async fn should_block(&self, url: &ValidatedUrl) -> bool;
    async fn update_blocklists(&self) -> Result<()>;
    fn get_blocked_count(&self) -> usize;
}
