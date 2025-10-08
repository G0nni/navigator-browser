// UI Layer - Visual rendering and window management
pub mod window;
pub mod renderer;
pub mod text_renderer;
pub mod address_bar;

pub use window::BrowserWindow;
pub use renderer::Renderer;
pub use address_bar::{AddressBar, AddressBarAction};
