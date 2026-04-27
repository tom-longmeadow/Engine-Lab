use base::ui::{text::style::TextStyle, widget::layout::{size::Size, text_measurer::TextMeasurer}};
use glyphon::cosmic_text;

use crate::renderer::pass::text::font::TextFontExt;


pub struct GlyphonTextMeasurer {
    font_system: glyphon::FontSystem,
}

impl GlyphonTextMeasurer {
    pub fn new() -> Self {
        let mut font_system = glyphon::FontSystem::new();
        for font in base::ui::text::font::TextFont::all() {
            font_system.db_mut().load_font_data(font.font_bytes().to_vec());
        }
        Self { font_system }
    }
}

impl TextMeasurer for GlyphonTextMeasurer {
    fn measure(&mut self, text: &str, style: &TextStyle) -> Size {
        let mut buffer = glyphon::Buffer::new(
            &mut self.font_system,
            glyphon::Metrics::new(style.font_size, style.line_height),
        );

        buffer.set_wrap(&mut self.font_system, cosmic_text::Wrap::None);
        buffer.set_size(&mut self.font_system, Some(100_000.0), None);
        buffer.set_text(
            &mut self.font_system,
            text,
            &style.font.attrs(),
            glyphon::Shaping::Advanced,
            None,
        );

        // API names can vary slightly by glyphon/cosmic_text version.
        let mut width = 0.0f32;
        let mut lines = 0usize;
        for run in buffer.layout_runs() {
            width = width.max(run.line_w);
            lines += 1;
        }

        Size {
            w: width,
            h: (lines.max(1) as f32) * style.line_height,
        }
    }
}