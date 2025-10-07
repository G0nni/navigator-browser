use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Orientation, Widget};
use std::cell::RefCell;
use std::rc::Rc;

/// URL bar widget with navigation controls
pub struct UrlBar {
    container: Box,
    entry: Entry,
    navigate_callback: Rc<RefCell<Option<Box<dyn Fn(String)>>>>,
}

impl UrlBar {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Horizontal, 4);
        container.set_margin_top(4);
        container.set_margin_bottom(4);
        container.set_margin_start(8);
        container.set_margin_end(8);

        // Back button
        let back_button = Button::builder()
            .icon_name("go-previous-symbolic")
            .tooltip_text("Go back")
            .build();

        // Forward button
        let forward_button = Button::builder()
            .icon_name("go-next-symbolic")
            .tooltip_text("Go forward")
            .build();

        // Refresh button
        let refresh_button = Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Reload")
            .build();

        // URL entry
        let entry = Entry::builder()
            .placeholder_text("Enter URL or search...")
            .hexpand(true)
            .build();

        // Go button
        let go_button = Button::builder()
            .icon_name("go-jump-symbolic")
            .tooltip_text("Navigate")
            .build();

        // Bookmark button
        let bookmark_button = Button::builder()
            .icon_name("bookmark-new-symbolic")
            .tooltip_text("Add bookmark")
            .build();

        // Menu button
        let menu_button = Button::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Menu")
            .build();

        container.append(&back_button);
        container.append(&forward_button);
        container.append(&refresh_button);
        container.append(&entry);
        container.append(&go_button);
        container.append(&bookmark_button);
        container.append(&menu_button);

        let navigate_callback: Rc<RefCell<Option<Box<dyn Fn(String)>>>> =
            Rc::new(RefCell::new(None));

        // Connect entry activation (Enter key)
        let entry_clone = entry.clone();
        let callback_clone = navigate_callback.clone();
        entry.connect_activate(move |_| {
            let text = entry_clone.text().to_string();
            if let Some(callback) = callback_clone.borrow().as_ref() {
                callback(text);
            }
        });

        // Connect go button
        let entry_clone = entry.clone();
        let callback_clone = navigate_callback.clone();
        go_button.connect_clicked(move |_| {
            let text = entry_clone.text().to_string();
            if let Some(callback) = callback_clone.borrow().as_ref() {
                callback(text);
            }
        });

        // Connect back button
        back_button.connect_clicked(|_| {
            tracing::info!("Back button clicked");
            // Navigation history will be implemented in full version
        });

        // Connect forward button
        forward_button.connect_clicked(|_| {
            tracing::info!("Forward button clicked");
            // Navigation history will be implemented in full version
        });

        // Connect refresh button
        refresh_button.connect_clicked(|_| {
            tracing::info!("Refresh button clicked");
            // Page refresh will be implemented in full version
        });

        // Connect bookmark button
        bookmark_button.connect_clicked(|_| {
            tracing::info!("Bookmark button clicked");
            // Bookmark functionality will be implemented in full version
        });

        // Connect menu button
        menu_button.connect_clicked(|_| {
            tracing::info!("Menu button clicked");
            // Menu will be implemented in full version
        });

        Self {
            container,
            entry,
            navigate_callback,
        }
    }

    /// Connect a callback for navigation events
    pub fn connect_navigate<F>(&self, callback: F)
    where
        F: Fn(String) + 'static,
    {
        *self.navigate_callback.borrow_mut() = Some(Box::new(callback));
    }

    /// Set the URL displayed in the bar
    pub fn set_url(&self, url: &str) {
        self.entry.set_text(url);
    }

    /// Get the current URL text
    pub fn get_url(&self) -> String {
        self.entry.text().to_string()
    }

    /// Set focus on the URL entry
    pub fn focus(&self) {
        self.entry.grab_focus();
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }
}

impl Default for UrlBar {
    fn default() -> Self {
        Self::new()
    }
}
