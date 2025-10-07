use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ListBox, Orientation, ScrolledWindow, Widget};

use crate::application::BrowserState;
use crate::domain::{Tab, TabId};
use std::sync::Arc;

/// Widget for displaying tabs vertically in a sidebar
pub struct VerticalTabsWidget {
    container: ScrolledWindow,
    list_box: ListBox,
    state: BrowserState,
}

impl VerticalTabsWidget {
    pub fn new(state: BrowserState) -> Self {
        // Create scrolled window for tab list
        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .min_content_width(200)
            .max_content_width(300)
            .build();

        // Create list box for tabs
        let list_box = ListBox::new();
        list_box.add_css_class("navigation-sidebar");
        list_box.set_selection_mode(gtk4::SelectionMode::Single);

        // Add vertical box for tabs and controls
        let main_box = Box::new(Orientation::Vertical, 0);

        // Add "New Tab" button at top
        let new_tab_button = Button::builder()
            .label("+ New Tab")
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(8)
            .margin_end(8)
            .build();

        let state_clone = state.clone();
        new_tab_button.connect_clicked(move |_| {
            let tab = Tab::new(state_clone.is_private_mode());
            let tab_id = state_clone.add_tab(tab);
            state_clone.set_active_tab(tab_id);
            tracing::info!("New tab created: {}", tab_id);
        });

        main_box.append(&new_tab_button);
        main_box.append(&list_box);

        container.set_child(Some(&main_box));

        let widget = Self {
            container,
            list_box,
            state,
        };

        // Initialize with one tab
        widget.add_initial_tab();

        widget
    }

    fn add_initial_tab(&self) {
        let tab = Tab::new(false);
        let tab_id = self.state.add_tab(tab);
        self.state.set_active_tab(tab_id);
        self.refresh();
    }

    /// Refresh the tab list from state
    pub fn refresh(&self) {
        // Clear existing items
        while let Some(child) = self.list_box.first_child() {
            self.list_box.remove(&child);
        }

        // Add all tabs from state
        let tabs = self.state.get_all_tabs();
        let active_tab_id = self.state.get_active_tab_id();

        for tab in tabs {
            let tab_row = self.create_tab_row(&tab, active_tab_id == Some(tab.id));
            self.list_box.append(&tab_row);
        }
    }

    fn create_tab_row(&self, tab: &Tab, is_active: bool) -> Box {
        let row = Box::new(Orientation::Horizontal, 8);
        row.set_margin_top(4);
        row.set_margin_bottom(4);
        row.set_margin_start(8);
        row.set_margin_end(8);

        if is_active {
            row.add_css_class("active-tab");
        }

        // Tab info container
        let info_box = Box::new(Orientation::Vertical, 2);
        info_box.set_hexpand(true);

        // Tab title
        let title_label = Label::new(Some(&tab.title));
        title_label.set_halign(gtk4::Align::Start);
        title_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        title_label.set_max_width_chars(25);
        info_box.append(&title_label);

        // Tab URL (if available)
        if let Some(url) = &tab.url {
            let url_label = Label::new(Some(url.as_str()));
            url_label.set_halign(gtk4::Align::Start);
            url_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            url_label.set_max_width_chars(25);
            url_label.add_css_class("dim-label");
            url_label.add_css_class("caption");
            info_box.append(&url_label);
        }

        row.append(&info_box);

        // Close button
        let close_button = Button::builder()
            .icon_name("window-close-symbolic")
            .valign(gtk4::Align::Center)
            .build();
        close_button.add_css_class("flat");
        close_button.add_css_class("circular");

        let state_clone = self.state.clone();
        let tab_id = tab.id;
        close_button.connect_clicked(move |_| {
            state_clone.remove_tab(tab_id);
            tracing::info!("Tab closed: {}", tab_id);
        });

        row.append(&close_button);

        // Make row clickable to switch tabs
        let gesture = gtk4::GestureClick::new();
        let state_clone = self.state.clone();
        let tab_id = tab.id;
        gesture.connect_released(move |_, _, _, _| {
            state_clone.set_active_tab(tab_id);
            tracing::info!("Switched to tab: {}", tab_id);
        });
        row.add_controller(gesture);

        row
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.container
    }

    /// Add a new tab
    pub fn add_tab(&self, tab: Tab) -> TabId {
        let tab_id = self.state.add_tab(tab);
        self.refresh();
        tab_id
    }

    /// Remove a tab
    pub fn remove_tab(&self, tab_id: TabId) {
        self.state.remove_tab(tab_id);
        self.refresh();
    }

    /// Set active tab
    pub fn set_active_tab(&self, tab_id: TabId) {
        self.state.set_active_tab(tab_id);
        self.refresh();
    }
}
