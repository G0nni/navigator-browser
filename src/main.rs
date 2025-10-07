// Navigator Browser - Custom Engine
// A secure web browser with custom HTML parser (no Chromium/Gecko)

pub mod domain;
pub mod application;
pub mod infrastructure;

use application::BrowserState;
use infrastructure::{SqliteDatabase, DefaultSecurityService, SecureNetworkClient, ServoRenderer};
use domain::{Tab, SecurityService, RenderingEngine};

use std::sync::Arc;
use tokio::sync::RwLock;

struct Navigator {
    browser_state: Arc<RwLock<BrowserState>>,
    db: Arc<SqliteDatabase>,
    security: Arc<DefaultSecurityService>,
    network: Arc<SecureNetworkClient>,
    renderer: Arc<ServoRenderer>,
}

impl Navigator {
    async fn new() -> anyhow::Result<Self> {
        tracing::info!("Initializing Navigator Browser...");

        let browser_state = Arc::new(RwLock::new(BrowserState::new()));
        let db = Arc::new(SqliteDatabase::new("navigator.db").await?);
        let security = Arc::new(DefaultSecurityService::new());
        let network = Arc::new(SecureNetworkClient::new()?);
        let renderer = Arc::new(ServoRenderer::new());

        // Create initial tab
        {
            let state = browser_state.write().await;
            let tab = Tab::new(false);
            state.add_tab(tab);
        }

        Ok(Self {
            browser_state,
            db,
            security,
            network,
            renderer,
        })
    }

    async fn navigate_to(&self, url_str: &str) -> anyhow::Result<()> {
        tracing::info!("Navigating to: {}", url_str);

        // Validate URL
        let validated_url = self.security.validate_url(url_str)?;

        // Check if blocked
        if self.security.is_blocked(&validated_url) {
            anyhow::bail!("This URL is blocked for security reasons");
        }

        // Load URL
        self.renderer.load_url(&validated_url).await?;

        // Get and display title
        let title = self.renderer.get_title().await?;
        println!("\n========================================");
        println!("Page Title: {}", title);
        println!("URL: {}", validated_url);
        println!("========================================\n");

        // Display rendered content (text mode for now)
        let content = self.renderer.render_to_text();
        println!("{}", content);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("navigator=info")
        .init();

    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║   Navigator - Custom Browser Engine (No Chromium) ║");
    println!("╚════════════════════════════════════════════════════╝\n");

    // Initialize Navigator
    let navigator = Arc::new(Navigator::new().await?);

    println!("Browser initialized successfully!");
    println!("Features:");
    println!("  ✓ Custom HTML parser (html5ever)");
    println!("  ✓ Security validation");
    println!("  ✓ HTTPS support");
    println!("  ✓ DOM parsing");
    println!("  ⚠ JavaScript: Not yet implemented");
    println!("  ⚠ CSS rendering: Not yet implemented");
    println!("  ⚠ Visual rendering: Text mode only\n");

    // Test navigation
    println!("Testing navigation to example.com...\n");

    if let Err(e) = navigator.navigate_to("https://example.com").await {
        eprintln!("Navigation error: {}", e);
    }

    println!("\n\nTesting navigation to Google...\n");

    if let Err(e) = navigator.navigate_to("https://www.google.com").await {
        eprintln!("Navigation error: {}", e);
    }

    println!("\n✅ Browser test completed!");
    println!("\nNext steps:");
    println!("  1. Add CSS parsing (cssparser)");
    println!("  2. Add layout engine");
    println!("  3. Add visual rendering (wgpu)");
    println!("  4. Add JavaScript engine (boa)");
    println!("  5. Add GUI with tabs (winit)\n");

    Ok(())
}
