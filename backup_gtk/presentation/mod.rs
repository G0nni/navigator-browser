// Presentation Layer - UI and user interaction
// GTK4-based interface with vertical tabs

pub mod browser_window;
pub mod vertical_tabs;
pub mod url_bar;

use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

use crate::application::BrowserState;
use crate::infrastructure::{DefaultSecurityService, SqliteDatabase, WebKitRenderer};
use std::sync::Arc;

pub use browser_window::BrowserWindow;
pub use vertical_tabs::VerticalTabsWidget;
pub use url_bar::UrlBar;

/// Main browser application
pub struct BrowserApplication {
    app: Application,
}

impl BrowserApplication {
    pub fn new() -> Result<Self> {
        let app = Application::builder()
            .application_id("com.navigator.browser")
            .build();

        app.connect_activate(move |app| {
            if let Err(e) = Self::on_activate(app) {
                tracing::error!("Failed to activate application: {}", e);
            }
        });

        Ok(Self { app })
    }

    fn on_activate(app: &Application) -> Result<()> {
        tracing::info!("Activating Navigator application");

        // Create browser window
        let window = BrowserWindow::new(app)?;
        window.show();

        Ok(())
    }

    pub fn run(self) {
        let args: Vec<String> = std::env::args().collect();
        self.app.run_with_args(&args);
    }
}
