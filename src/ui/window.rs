use winit::{
    event_loop::EventLoop,
    window::Window,
    dpi::LogicalSize,
};
use anyhow::Result;
use std::sync::Arc;

/// Browser window manager
pub struct BrowserWindow {
    window: Arc<Window>,
}

impl BrowserWindow {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        let window_attributes = Window::default_attributes()
            .with_title("Navigator - Custom Browser")
            .with_inner_size(LogicalSize::new(1400.0, 900.0))
            .with_min_inner_size(LogicalSize::new(800.0, 600.0));

        let window = Arc::new(event_loop.create_window(window_attributes)?);

        Ok(Self { window })
    }

    pub fn window(&self) -> Arc<Window> {
        self.window.clone()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
