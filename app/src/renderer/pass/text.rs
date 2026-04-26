pub mod style;
pub mod params;
pub mod item;
pub mod font;

use glyphon::cosmic_text;
use crate::renderer::pass::{
    RenderPass,
    text::{
        font::TextFont,
        item::TextItem,
        params::TextParams,
        style::TextStyle,
    },
};

struct GlyphonState {
    width: u32,
    height: u32,
    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    renderer: glyphon::TextRenderer,
    buffers: Vec<glyphon::Buffer>,
}

pub struct TextRenderPass {
    params: TextParams,
    state: Option<GlyphonState>,
}

impl TextRenderPass {
    pub fn new(params: TextParams) -> Self {
        Self { params, state: None }
    }

    fn style_for<'a>(&'a self, item: &'a TextItem) -> &'a TextStyle {
        item.style.as_ref().unwrap_or(&self.params.default_style)
    }

    fn rebuild_buffers(
        &mut self,
        font_system: &mut glyphon::FontSystem,
        width: u32,
        height: u32,
    ) -> Vec<glyphon::Buffer> {
        let mut out = Vec::with_capacity(self.params.items.len());

        for item in &self.params.items {
            let style = self.style_for(item);
            let attrs = style.font.attrs();

            let mut buffer = glyphon::Buffer::new(
                font_system,
                glyphon::Metrics::new(style.font_size, style.line_height),
            );

            buffer.set_size(font_system, Some(width as f32), Some(height as f32));
            buffer.set_text(
                font_system,
                &item.text,
                &attrs,
                glyphon::Shaping::Advanced,
                Some(cosmic_text::Align::Left),
            );

            out.push(buffer);
        }

        out
    }
}

impl RenderPass for TextRenderPass {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) {
        if let Some(state) = &mut self.state {
            state.width = config.width;
            state.height = config.height;

            for buffer in &mut state.buffers {
                buffer.set_size(
                    &mut state.font_system,
                    Some(config.width as f32),
                    Some(config.height as f32),
                );
            }

            state.viewport.update(
                queue,
                glyphon::Resolution { width: config.width, height: config.height },
            );
            return;
        }

        let mut font_system = glyphon::FontSystem::new();

        // Load all font variants
        for variant in TextFont::all() {
            font_system.db_mut().load_font_data(variant.font_bytes().to_vec());
        }

        let swash_cache = glyphon::SwashCache::new();
        let cache = glyphon::Cache::new(device);
        let mut atlas = glyphon::TextAtlas::new(device, queue, &cache, config.format);
        let viewport = glyphon::Viewport::new(device, &cache);
        let renderer = glyphon::TextRenderer::new(
            &mut atlas,
            device,
            wgpu::MultisampleState::default(),
            None,
        );

        let buffers = self.rebuild_buffers(&mut font_system, config.width, config.height);

        let mut state = GlyphonState {
            width: config.width,
            height: config.height,
            font_system,
            swash_cache,
            viewport,
            atlas,
            renderer,
            buffers,
        };

        state.viewport.update(
            queue,
            glyphon::Resolution { width: config.width, height: config.height },
        );

        self.state = Some(state);
    }

    fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let default_style = self.params.default_style.clone();
        let items = &self.params.items;

        let Some(state) = &mut self.state else { return; };

        state.viewport.update(
            queue,
            glyphon::Resolution { width: state.width, height: state.height },
        );

        let mut areas = Vec::with_capacity(items.len());
        for (i, item) in items.iter().enumerate() {
            if i >= state.buffers.len() {
                break;
            }

            let style = item.style.as_ref().unwrap_or(&default_style);
            let [r, g, b, a] = style.color;

            areas.push(glyphon::TextArea {
                buffer: &state.buffers[i],
                left: item.x,
                top: item.y,
                scale: 1.0,
                bounds: glyphon::TextBounds {
                    left: 0,
                    top: 0,
                    right: state.width as i32,
                    bottom: state.height as i32,
                },
                default_color: cosmic_text::Color::rgba(r, g, b, a),
                custom_glyphs: &[],
            });
        }

        state
            .renderer
            .prepare(
                device,
                queue,
                &mut state.font_system,
                &mut state.atlas,
                &state.viewport,
                areas,
                &mut state.swash_cache,
            )
            .expect("glyphon prepare failed");
    }

    fn draw<'a>(&'a mut self, pass: &mut wgpu::RenderPass<'a>) {
        let Some(state) = &mut self.state else { return; };
        state
            .renderer
            .render(&state.atlas, &state.viewport, pass)
            .expect("glyphon render failed");
    }
}
 