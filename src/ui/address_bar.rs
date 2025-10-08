use glyphon::{
    Attrs, Buffer, Color as GlyphonColor, Family, FontSystem, Metrics, Shaping,
};
use winit::keyboard::{Key, NamedKey};

/// Address bar for URL input
pub struct AddressBar {
    url: String,
    is_focused: bool,
    cursor_position: usize,
}

impl AddressBar {
    pub fn new() -> Self {
        Self {
            url: String::from("https://example.com"),
            is_focused: true,
            cursor_position: 0,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
        self.cursor_position = self.url.len();
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: &Key, text: Option<&str>) -> Option<AddressBarAction> {
        match key {
            Key::Named(NamedKey::Enter) => {
                return Some(AddressBarAction::Navigate(self.url.clone()));
            }
            Key::Named(NamedKey::Backspace) => {
                if !self.url.is_empty() && self.cursor_position > 0 {
                    self.url.remove(self.cursor_position - 1);
                    self.cursor_position -= 1;
                }
            }
            Key::Named(NamedKey::Delete) => {
                if self.cursor_position < self.url.len() {
                    self.url.remove(self.cursor_position);
                }
            }
            Key::Named(NamedKey::ArrowLeft) => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            Key::Named(NamedKey::ArrowRight) => {
                if self.cursor_position < self.url.len() {
                    self.cursor_position += 1;
                }
            }
            Key::Named(NamedKey::Home) => {
                self.cursor_position = 0;
            }
            Key::Named(NamedKey::End) => {
                self.cursor_position = self.url.len();
            }
            Key::Character(_) => {
                if let Some(text) = text {
                    self.url.insert_str(self.cursor_position, text);
                    self.cursor_position += text.len();
                }
            }
            _ => {}
        }
        None
    }

    /// Create a text buffer for rendering the address bar
    pub fn create_buffer(&self, font_system: &mut FontSystem, width: f32) -> Buffer {
        let metrics = Metrics::new(18.0, 22.0);
        let mut buffer = Buffer::new(font_system, metrics);

        buffer.set_size(font_system, Some(width - 40.0), Some(40.0));

        let display_text = if self.is_focused {
            format!("{}|", self.url)
        } else {
            self.url.clone()
        };

        buffer.set_text(
            font_system,
            &display_text,
            Attrs::new().family(Family::Monospace),
            Shaping::Advanced,
        );

        buffer
    }

    pub fn background_color(&self) -> [f32; 3] {
        if self.is_focused {
            [1.0, 1.0, 1.0] // White when focused
        } else {
            [0.95, 0.95, 0.95] // Light gray when not focused
        }
    }

    pub fn text_color(&self) -> GlyphonColor {
        GlyphonColor::rgb(0, 0, 0)
    }
}

impl Default for AddressBar {
    fn default() -> Self {
        Self::new()
    }
}

pub enum AddressBarAction {
    Navigate(String),
}
