// Navigator Browser - Visual Edition
// A secure web browser with custom HTML parser and GPU rendering

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod ui;

use application::BrowserState;
use infrastructure::{SqliteDatabase, DefaultSecurityService, SecureNetworkClient, ServoRenderer};
use domain::{Tab, SecurityService, RenderingEngine};
use ui::{BrowserWindow, Renderer};

use std::sync::Arc;
use tokio::sync::RwLock;
use winit::{
    event::{Event, WindowEvent, KeyEvent},
    event_loop::{EventLoop, ControlFlow},
    keyboard::{Key, NamedKey},
};

struct Navigator {
    browser_state: Arc<RwLock<BrowserState>>,
    db: Arc<SqliteDatabase>,
    security: Arc<DefaultSecurityService>,
    network: Arc<SecureNetworkClient>,
    html_renderer: Arc<ServoRenderer>,
    current_html: Arc<RwLock<String>>,
}

impl Navigator {
    async fn new() -> anyhow::Result<Self> {
        tracing::info!("Initializing Navigator Browser...");

        let browser_state = Arc::new(RwLock::new(BrowserState::new()));
        let db = Arc::new(SqliteDatabase::new("navigator.db").await?);
        let security = Arc::new(DefaultSecurityService::new());
        let network = Arc::new(SecureNetworkClient::new()?);
        let html_renderer = Arc::new(ServoRenderer::new());

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
            html_renderer,
            current_html: Arc::new(RwLock::new(String::new())),
        })
    }

    async fn navigate_to(&self, url_str: &str) -> anyhow::Result<String> {
        tracing::info!("Navigating to: {}", url_str);

        // Validate URL
        let validated_url = self.security.validate_url(url_str)?;

        // Check if blocked
        if self.security.is_blocked(&validated_url) {
            anyhow::bail!("This URL is blocked for security reasons");
        }

        // Load URL
        self.html_renderer.load_url(&validated_url).await?;

        // Get rendered content
        let content = self.html_renderer.render_to_text();

        // Update current HTML
        {
            let mut current = self.current_html.write().await;
            *current = content.clone();
        }

        // Get title
        let title = self.html_renderer.get_title().await?;
        tracing::info!("Page loaded: {} - {}", title, validated_url);

        Ok(content)
    }

    fn get_current_html(&self) -> String {
        if let Ok(html) = self.current_html.try_read() {
            html.clone()
        } else {
            String::from("Loading...")
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("navigator=info,wgpu=warn")
        .init();

    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║   Navigator - Visual Browser (Phase 2: GPU Rendering)║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // Create event loop
    let event_loop = EventLoop::new()?;

    // Run async initialization
    let runtime = tokio::runtime::Runtime::new()?;
    let navigator = runtime.block_on(async {
        Navigator::new().await
    })?;
    let navigator = Arc::new(navigator);

    // Load default page
    let nav_clone = navigator.clone();
    runtime.spawn(async move {
        if let Err(e) = nav_clone.navigate_to("https://example.com").await {
            tracing::error!("Failed to load default page: {}", e);
        }
    });

    // Create window
    let window = BrowserWindow::new(&event_loop)?;

    // Create renderer
    let mut renderer = runtime.block_on(async {
        Renderer::new(window.window()).await
    })?;

    println!("✓ Window created");
    println!("✓ GPU renderer initialized");
    println!("✓ Loading example.com...\n");
    println!("Controls:");
    println!("  F5 - Reload");
    println!("  ESC - Quit\n");

    // Event loop
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    tracing::info!("Close requested, exiting...");
                    elwt.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    tracing::debug!("Window resized to: {:?}", physical_size);
                    renderer.resize(physical_size);
                    window.request_redraw();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    handle_keyboard_input(&event, &navigator, &runtime, &window);
                }
                WindowEvent::RedrawRequested => {
                    let html = navigator.get_current_html();
                    if let Err(e) = renderer.render(&html) {
                        tracing::error!("Render error: {}", e);
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}

fn handle_keyboard_input(
    event: &KeyEvent,
    navigator: &Arc<Navigator>,
    runtime: &tokio::runtime::Runtime,
    window: &BrowserWindow
) {
    if !event.state.is_pressed() {
        return;
    }

    match event.logical_key {
        Key::Named(NamedKey::F5) => {
            tracing::info!("Refresh requested");
            let nav_clone = navigator.clone();
            runtime.spawn(async move {
                if let Err(e) = nav_clone.navigate_to("https://example.com").await {
                    tracing::error!("Navigation error: {}", e);
                }
            });
            window.request_redraw();
        }
        Key::Named(NamedKey::Escape) => {
            tracing::info!("Escape pressed - use window close button to quit");
        }
        _ => {}
    }
}
