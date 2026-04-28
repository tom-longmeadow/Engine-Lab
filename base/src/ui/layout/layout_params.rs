use crate::ui::{
 
    layout::{
        border::BorderStyle,
        color::Color,
        edge_insets::EdgeInsets,
    }, text::style::TextStyle,
};

#[derive(Clone, Copy, Debug)]
pub struct LayoutParams {
    pub control_padding: EdgeInsets,
    pub panel_padding: EdgeInsets,
    pub gap: f32,
    pub text: TextStyle,
    pub background: Color,
    pub border: BorderStyle,
}

impl Default for LayoutParams {
    fn default() -> Self {
        Self {
            control_padding: EdgeInsets::all(6.0),
            panel_padding: EdgeInsets::all(8.0),
            gap: 6.0,
            text: TextStyle::default(),
            background: Color::TRANSPARENT,
            border: BorderStyle::default(),
        }
    }
}

impl LayoutParams {
    pub fn with_text(mut self, style: TextStyle) -> Self {
        self.text = style;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn with_control_padding(mut self, padding: EdgeInsets) -> Self {
        self.control_padding = padding;
        self
    }

    pub fn with_panel_padding(mut self, padding: EdgeInsets) -> Self {
        self.panel_padding = padding;
        self
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    pub fn with_border(mut self, border: BorderStyle) -> Self {
        self.border = border;
        self
    }
}