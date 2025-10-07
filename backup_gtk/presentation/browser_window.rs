use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, HeaderBar, Orientation, Paned};

use super::{UrlBar, VerticalTabsWidget};
use crate::application::BrowserState;
use crate::infrastructure::{DefaultSecurityService, SqliteDatabase, WebKitRenderer};
use std::sync::Arc;

/// Main browser window with vertical tabs layout
pub struct BrowserWindow {
    window: ApplicationWindow,
    state: BrowserState,
}

impl BrowserWindow {
    pub fn new(app: &Application) -> Result<Self> {
        // Initialize browser state
        let state = BrowserState::new();

        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Navigator")
            .default_width(1200)
            .default_height(800)
            .build();

        // Create header bar
        let header_bar = HeaderBar::new();
        header_bar.set_show_title_buttons(true);
        window.set_titlebar(Some(&header_bar));

        // Create main layout container
        let main_box = Box::new(Orientation::Vertical, 0);

        // Create URL bar
        let url_bar = UrlBar::new();
        main_box.append(&url_bar.widget());

        // Create paned container for vertical tabs and content
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_vexpand(true);

        // Create vertical tabs sidebar
        let tabs_widget = VerticalTabsWidget::new(state.clone());
        paned.set_start_child(Some(&tabs_widget.widget()));
        paned.set_shrink_start_child(false);
        paned.set_resize_start_child(false);

        // Create web view container
        let webview_box = Box::new(Orientation::Vertical, 0);
        webview_box.set_hexpand(true);
        webview_box.set_vexpand(true);

        // Create initial web view
        let renderer = WebKitRenderer::new();
        webview_box.append(renderer.get_webview());

        paned.set_end_child(Some(&webview_box));

        main_box.append(&paned);
        window.set_child(Some(&main_box));

        // Connect URL bar signals
        let state_clone = state.clone();
        url_bar.connect_navigate(move |url_str| {
            tracing::info!("Navigate requested: {}", url_str);
            // This will be handled by use cases in a full implementation
        });

        // Connect keyboard shortcuts
        Self::setup_keyboard_shortcuts(&window, &state);

        Ok(Self { window, state })
    }

    fn setup_keyboard_shortcuts(window: &ApplicationWindow, state: &BrowserState) {
        // Add keyboard shortcuts
        // Ctrl+T: New tab
        // Ctrl+W: Close tab
        // Ctrl+L: Focus URL bar
        // Ctrl+Shift+P: Private mode
        // etc.

        // This is a placeholder - full implementation would use GtkEventController
        tracing::debug!("Keyboard shortcuts initialized");
    }

    pub fn show(&self) {
        self.window.present();
    }

    pub fn window(&self) -> &ApplicationWindow {
        &self.window
    }

    pub fn state(&self) -> &BrowserState {
        &self.state
    }
}
