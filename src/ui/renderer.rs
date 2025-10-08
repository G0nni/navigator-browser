use wgpu::{
    Device, Queue, Surface, SurfaceConfiguration,
};
use winit::window::Window;
use anyhow::Result;
use std::sync::Arc;
use super::text_renderer::TextRenderer;
use super::address_bar::AddressBar;
use glyphon::{TextArea, TextBounds, Color as GlyphonColor};

const ADDRESS_BAR_HEIGHT: f32 = 50.0;

/// GPU renderer using wgpu
pub struct Renderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    text_renderer: TextRenderer,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface
        let surface = instance.create_surface(window)?;

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find an appropriate adapter"))?;

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Create text renderer
        let text_renderer = TextRenderer::new(
            &device,
            &queue,
            surface_format,
            size.width,
            size.height,
        )?;

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            text_renderer,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.text_renderer.resize(&self.device, &self.queue, new_size.width, new_size.height);
        }
    }

    pub fn render(&mut self, html_content: &str, address_bar: &AddressBar) -> Result<()> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Clear background
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Background Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.95,
                            g: 0.95,
                            b: 0.95,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // Create buffers (must live until render call)
        let address_bar_buffer = address_bar.create_buffer(
            self.text_renderer.font_system(),
            self.size.width as f32,
        );

        let content_buffer = if !html_content.is_empty() {
            Some(self.text_renderer.create_buffer(
                html_content,
                14.0,
                self.size.width,
                self.size.height - ADDRESS_BAR_HEIGHT as u32
            ))
        } else {
            None
        };

        // Build text areas
        let mut text_areas = Vec::new();

        // Address bar
        text_areas.push(TextArea {
            buffer: &address_bar_buffer,
            left: 10.0,
            top: 10.0,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: self.size.width as i32,
                bottom: ADDRESS_BAR_HEIGHT as i32,
            },
            default_color: address_bar.text_color(),
            custom_glyphs: &[],
        });

        // Page content
        if let Some(ref buffer) = content_buffer {
            text_areas.push(TextArea {
                buffer,
                left: 20.0,
                top: ADDRESS_BAR_HEIGHT + 20.0,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: ADDRESS_BAR_HEIGHT as i32,
                    right: self.size.width as i32,
                    bottom: self.size.height as i32,
                },
                default_color: GlyphonColor::rgb(0, 0, 0),
                custom_glyphs: &[],
            });
        }

        // Render all text
        self.text_renderer.render(
            &self.device,
            &self.queue,
            &view,
            &mut encoder,
            text_areas,
        )?;

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }
}
