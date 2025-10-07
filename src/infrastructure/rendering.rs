use crate::domain::{RenderingEngine, ValidatedUrl};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{RcDom, Handle, NodeData};

/// Custom browser rendering engine using html5ever
pub struct ServoRenderer {
    current_url: Arc<Mutex<Option<ValidatedUrl>>>,
    current_html: Arc<Mutex<String>>,
    current_title: Arc<Mutex<String>>,
}

impl ServoRenderer {
    pub fn new() -> Self {
        Self {
            current_url: Arc::new(Mutex::new(None)),
            current_html: Arc::new(Mutex::new(String::new())),
            current_title: Arc::new(Mutex::new("Navigator".to_string())),
        }
    }

    /// Fetch HTML content from URL
    async fn fetch_html(&self, url: &ValidatedUrl) -> Result<String> {
        tracing::info!("Fetching HTML from: {}", url);

        let client = reqwest::Client::builder()
            .user_agent(format!("Navigator/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        let response = client.get(url.as_str()).send().await?;
        let html = response.text().await?;

        tracing::info!("Received {} bytes of HTML", html.len());
        Ok(html)
    }

    /// Parse HTML into DOM
    fn parse_html(&self, html: &str) -> RcDom {
        tracing::info!("Parsing HTML ({} bytes)", html.len());
        parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .unwrap()
    }

    /// Extract title from DOM
    fn extract_title(&self, dom: &RcDom) -> String {
        fn walk(handle: &Handle, title: &mut Option<String>) {
            let node = handle;
            match &node.data {
                NodeData::Element { name, .. } => {
                    if &name.local == "title" {
                        if let Some(text_node) = node.children.borrow().first() {
                            if let NodeData::Text { contents } = &text_node.data {
                                *title = Some(contents.borrow().to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
            for child in node.children.borrow().iter() {
                walk(child, title);
            }
        }

        let mut title = None;
        walk(&dom.document, &mut title);
        title.unwrap_or_else(|| "Untitled".to_string())
    }

    /// Render DOM to text (simple rendering for now)
    pub fn render_to_text(&self) -> String {
        if let Ok(html) = self.current_html.lock() {
            // Parse HTML on demand
            let dom = self.parse_html(&html);
            let mut output = String::new();
            self.walk_dom(&dom.document, &mut output, 0);
            output
        } else {
            String::new()
        }
    }

    fn walk_dom(&self, handle: &Handle, output: &mut String, depth: usize) {
        let node = handle;
        let indent = "  ".repeat(depth);

        match &node.data {
            NodeData::Document => {}
            NodeData::Element { name, .. } => {
                let tag_name = &name.local;
                output.push_str(&format!("{}<{}>\n", indent, tag_name));
            }
            NodeData::Text { contents } => {
                let text = contents.borrow();
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    output.push_str(&format!("{}{}\n", indent, trimmed));
                }
            }
            _ => {}
        }

        for child in node.children.borrow().iter() {
            self.walk_dom(child, output, depth + 1);
        }
    }
}

impl Default for ServoRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RenderingEngine for ServoRenderer {
    async fn load_url(&self, url: &ValidatedUrl) -> Result<()> {
        tracing::info!("Loading URL: {}", url);

        // Fetch HTML
        let html = self.fetch_html(url).await?;

        // Parse HTML
        let dom = self.parse_html(&html);

        // Extract title
        let title = self.extract_title(&dom);

        // Update state
        if let Ok(mut current_url) = self.current_url.lock() {
            *current_url = Some(url.clone());
        }
        if let Ok(mut current_html) = self.current_html.lock() {
            *current_html = html.clone();
        }
        if let Ok(mut current_title) = self.current_title.lock() {
            *current_title = title;
        }

        tracing::info!("Page loaded successfully: {}", url);
        Ok(())
    }

    async fn get_title(&self) -> Result<String> {
        if let Ok(title) = self.current_title.lock() {
            Ok(title.clone())
        } else {
            Ok("Navigator".to_string())
        }
    }

    async fn execute_javascript(&self, script: &str) -> Result<String> {
        tracing::debug!("JavaScript execution: {}", script);

        // TODO: Integrate boa_engine for JS execution
        // For now, just log
        tracing::warn!("JavaScript execution not yet implemented");
        Ok(String::new())
    }

    async fn take_screenshot(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

/// Rendering configuration
#[derive(Debug, Clone)]
pub struct RenderingConfig {
    pub enable_javascript: bool,
    pub enable_images: bool,
    pub enable_plugins: bool,
    pub user_agent: Option<String>,
    pub default_encoding: String,
}

impl Default for RenderingConfig {
    fn default() -> Self {
        Self {
            enable_javascript: true,
            enable_images: true,
            enable_plugins: false,
            user_agent: Some(format!("Navigator/{}", env!("CARGO_PKG_VERSION"))),
            default_encoding: "UTF-8".to_string(),
        }
    }
}
