use crate::domain::{Certificate, NetworkService, SecurityContext, ValidatedUrl};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;

/// HTTP client with security features
pub struct SecureNetworkClient {
    client: Client,
}

impl SecureNetworkClient {
    pub fn new() -> Result<Self> {
        // Configure client with security best practices
        let client = Client::builder()
            .use_rustls_tls() // Use Rust's memory-safe TLS implementation
            .https_only(false) // Allow HTTP but we'll enforce HTTPS at higher level
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(std::time::Duration::from_secs(30))
            .user_agent(format!("Navigator/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client })
    }
}

impl Default for SecureNetworkClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default network client")
    }
}

#[async_trait]
impl NetworkService for SecureNetworkClient {
    async fn fetch(&self, url: &ValidatedUrl) -> Result<Vec<u8>> {
        tracing::debug!("Fetching URL: {}", url);

        let response = self
            .client
            .get(url.as_str())
            .send()
            .await
            .context("Failed to send HTTP request")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ));
        }

        let bytes = response
            .bytes()
            .await
            .context("Failed to read response body")?
            .to_vec();

        Ok(bytes)
    }

    async fn verify_certificate(&self, url: &ValidatedUrl) -> Result<Certificate> {
        // For HTTPS URLs, verify certificate
        if !url.is_secure() {
            return Err(anyhow!("Cannot verify certificate for non-HTTPS URL"));
        }

        // Make a request to verify the certificate
        let response = self
            .client
            .get(url.as_str())
            .send()
            .await
            .context("Failed to verify certificate")?;

        // In a real implementation, we would extract actual certificate details
        // For now, return a mock certificate
        Ok(Certificate {
            subject: url.host_str().unwrap_or("unknown").to_string(),
            issuer: "Unknown CA".to_string(),
            valid_from: chrono::Utc::now(),
            valid_until: chrono::Utc::now() + chrono::Duration::days(365),
            is_valid: response.status().is_success(),
        })
    }

    async fn check_security(&self, url: &ValidatedUrl) -> Result<SecurityContext> {
        let mut context = SecurityContext::new();

        if url.is_secure() {
            match self.verify_certificate(url).await {
                Ok(cert) => {
                    context.is_secure = true;
                    context.certificate = Some(cert);
                }
                Err(e) => {
                    tracing::warn!("Certificate verification failed: {}", e);
                    context.is_secure = false;
                }
            }
        }

        Ok(context)
    }
}

/// DNS-over-HTTPS resolver for enhanced privacy
pub struct DohResolver {
    client: Client,
    doh_server: String,
}

impl DohResolver {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .use_rustls_tls()
            .build()
            .context("Failed to create DoH client")?;

        Ok(Self {
            client,
            doh_server: "https://cloudflare-dns.com/dns-query".to_string(),
        })
    }

    pub async fn resolve(&self, domain: &str) -> Result<Vec<std::net::IpAddr>> {
        tracing::debug!("Resolving domain via DoH: {}", domain);

        // In a real implementation, we would make a DNS query over HTTPS
        // For now, return an empty result as this is a stub
        Ok(Vec::new())
    }
}

impl Default for DohResolver {
    fn default() -> Self {
        Self::new().expect("Failed to create default DoH resolver")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_client_creation() {
        let client = SecureNetworkClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_doh_resolver_creation() {
        let resolver = DohResolver::new();
        assert!(resolver.is_ok());
    }
}
