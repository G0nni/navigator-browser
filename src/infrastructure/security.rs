use crate::domain::{SecurityService, ValidatedUrl};
use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::sync::RwLock;

/// Default implementation of SecurityService
pub struct DefaultSecurityService {
    blocked_domains: RwLock<HashSet<String>>,
    allow_mixed_content: bool,
}

impl DefaultSecurityService {
    pub fn new() -> Self {
        let mut blocked = HashSet::new();

        // Add some example blocked domains (malware, phishing)
        // In production, this would be loaded from a regularly updated list
        blocked.insert("malware-example.com".to_string());
        blocked.insert("phishing-example.com".to_string());

        Self {
            blocked_domains: RwLock::new(blocked),
            allow_mixed_content: false,
        }
    }

    pub fn add_blocked_domain(&self, domain: String) {
        if let Ok(mut blocked) = self.blocked_domains.write() {
            blocked.insert(domain);
        }
    }

    pub fn remove_blocked_domain(&self, domain: &str) {
        if let Ok(mut blocked) = self.blocked_domains.write() {
            blocked.remove(domain);
        }
    }
}

impl Default for DefaultSecurityService {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityService for DefaultSecurityService {
    fn validate_url(&self, url: &str) -> Result<ValidatedUrl> {
        // Trim whitespace
        let trimmed = url.trim();

        // If no scheme, assume HTTPS (secure by default)
        let url_with_scheme = if !trimmed.contains("://") {
            format!("https://{}", trimmed)
        } else {
            trimmed.to_string()
        };

        // Parse URL
        let parsed = ValidatedUrl::parse(&url_with_scheme)
            .map_err(|e| anyhow!("Invalid URL: {}", e))?;

        // Block non-HTTP(S) schemes for security (except about:, data: for specific cases)
        match parsed.scheme() {
            "http" | "https" => Ok(parsed),
            "about" | "data" => Ok(parsed),
            scheme => Err(anyhow!("Unsupported URL scheme: {}", scheme)),
        }
    }

    fn is_blocked(&self, url: &ValidatedUrl) -> bool {
        if let Some(host) = url.host_str() {
            if let Ok(blocked) = self.blocked_domains.read() {
                return blocked.contains(host);
            }
        }
        false
    }

    fn sanitize_html(&self, html: &str) -> String {
        // Basic HTML sanitization
        // In production, use a proper HTML sanitizer library
        html.replace("<script", "&lt;script")
            .replace("javascript:", "")
            .replace("onerror=", "")
            .replace("onclick=", "")
            .replace("onload=", "")
    }

    fn allow_mixed_content(&self, url: &ValidatedUrl) -> bool {
        // Only allow mixed content if explicitly enabled and URL is secure
        self.allow_mixed_content && url.is_secure()
    }
}

/// Content Security Policy builder
pub struct CspBuilder {
    directives: Vec<String>,
}

impl CspBuilder {
    pub fn new() -> Self {
        Self {
            directives: Vec::new(),
        }
    }

    pub fn default_src(mut self, sources: &[&str]) -> Self {
        self.directives.push(format!("default-src {}", sources.join(" ")));
        self
    }

    pub fn script_src(mut self, sources: &[&str]) -> Self {
        self.directives.push(format!("script-src {}", sources.join(" ")));
        self
    }

    pub fn style_src(mut self, sources: &[&str]) -> Self {
        self.directives.push(format!("style-src {}", sources.join(" ")));
        self
    }

    pub fn img_src(mut self, sources: &[&str]) -> Self {
        self.directives.push(format!("img-src {}", sources.join(" ")));
        self
    }

    pub fn upgrade_insecure_requests(mut self) -> Self {
        self.directives.push("upgrade-insecure-requests".to_string());
        self
    }

    pub fn build(self) -> String {
        self.directives.join("; ")
    }
}

impl Default for CspBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Default strict CSP for the browser
pub fn default_csp() -> String {
    CspBuilder::new()
        .default_src(&["'self'"])
        .script_src(&["'self'", "'unsafe-inline'", "'unsafe-eval'"]) // Needed for some sites
        .style_src(&["'self'", "'unsafe-inline'"])
        .img_src(&["'self'", "data:", "https:"])
        .upgrade_insecure_requests()
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_adds_https() {
        let service = DefaultSecurityService::new();
        let result = service.validate_url("example.com").unwrap();
        assert_eq!(result.as_str(), "https://example.com/");
    }

    #[test]
    fn test_validate_url_preserves_http() {
        let service = DefaultSecurityService::new();
        let result = service.validate_url("http://example.com").unwrap();
        assert_eq!(result.as_str(), "http://example.com/");
    }

    #[test]
    fn test_blocked_domain() {
        let service = DefaultSecurityService::new();
        service.add_blocked_domain("badsite.com".to_string());

        let url = ValidatedUrl::parse("https://badsite.com").unwrap();
        assert!(service.is_blocked(&url));
    }

    #[test]
    fn test_sanitize_html() {
        let service = DefaultSecurityService::new();
        let malicious = "<script>alert('xss')</script>";
        let sanitized = service.sanitize_html(malicious);
        assert!(!sanitized.contains("<script"));
    }

    #[test]
    fn test_csp_builder() {
        let csp = CspBuilder::new()
            .default_src(&["'self'"])
            .script_src(&["'self'", "'unsafe-inline'"])
            .build();

        assert!(csp.contains("default-src 'self'"));
        assert!(csp.contains("script-src 'self' 'unsafe-inline'"));
    }
}
