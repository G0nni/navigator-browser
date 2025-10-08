use glyphon::{
    Attrs, Buffer, Family, FontSystem, Metrics, Resolution, Shaping,
    SwashCache, TextArea, TextAtlas, TextRenderer as GlyphonTextRenderer, Viewport,
};
use wgpu::{Device, Queue, MultisampleState, TextureFormat};
use anyhow::Result;

/// Text rendering system using glyphon
pub struct TextRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: TextAtlas,
    text_renderer: GlyphonTextRenderer,
    viewport: Viewport,
}

impl TextRenderer {
    pub fn new(
        device: &Device,
        queue: &Queue,
        format: TextureFormat,
        _width: u32,
        _height: u32,
    ) -> Result<Self> {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = glyphon::Cache::new(device);
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let text_renderer = GlyphonTextRenderer::new(
            &mut atlas,
            device,
            MultisampleState::default(),
            None,
        );

        let viewport = Viewport::new(device, &cache);

        Ok(Self {
            font_system,
            swash_cache,
            atlas,
            text_renderer,
            viewport,
        })
    }

    pub fn resize(&mut self, _device: &Device, queue: &Queue, width: u32, height: u32) {
        self.viewport.update(
            queue,
            Resolution {
                width,
                height,
            },
        );
    }

    /// Create a text buffer for rendering
    pub fn create_buffer(&mut self, text: &str, font_size: f32, width: u32, height: u32) -> Buffer {
        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);

        buffer.set_size(
            &mut self.font_system,
            Some(width as f32),
            Some(height as f32),
        );

        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );

        buffer
    }

    /// Render text buffers to screen
    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        text_areas: Vec<TextArea>,
    ) -> Result<()> {
        // Prepare text atlas
        self.text_renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                text_areas,
                &mut self.swash_cache,
            )
            .map_err(|e| anyhow::anyhow!("Failed to prepare text: {:?}", e))?;

        // Render text
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Text Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // Don't clear, we're drawing on top
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.text_renderer
                .render(&self.atlas, &self.viewport, &mut pass)
                .map_err(|e| anyhow::anyhow!("Failed to render text: {:?}", e))?;
        }

        Ok(())
    }

    pub fn font_system(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }
}
