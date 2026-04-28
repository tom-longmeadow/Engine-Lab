
use crate::ui::layout::{
    border::BorderStyle,
    color::Color,
    edge_insets::EdgeInsets,
    rect::Rect,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoxModel {
    rect: Rect,
    padding: Option<EdgeInsets>,
    background: Option<Color>,
    border: Option<BorderStyle>,
}

impl BoxModel {
    pub fn rect(&self) -> Rect { self.rect }
    pub fn set_rect(&mut self, rect: Rect) { self.rect = rect; }

    pub fn padding(&self) -> Option<EdgeInsets> { self.padding }
    pub fn set_padding(&mut self, padding: EdgeInsets) { self.padding = Some(padding); }
    pub fn clear_padding(&mut self) { self.padding = None; }
    pub fn resolved_padding(&self, fallback: EdgeInsets) -> EdgeInsets {
        self.padding.unwrap_or(fallback)
    }

    pub fn background(&self) -> Option<Color> { self.background }
    pub fn set_background(&mut self, background: Color) { self.background = Some(background); }
    pub fn clear_background(&mut self) { self.background = None; }
    pub fn resolved_background(&self, fallback: Color) -> Color {
        self.background.unwrap_or(fallback)
    }

    pub fn border(&self) -> Option<BorderStyle> { self.border }
    pub fn set_border(&mut self, border: BorderStyle) { self.border = Some(border); }
    pub fn clear_border(&mut self) { self.border = None; }
    pub fn resolved_border(&self, fallback: BorderStyle) -> BorderStyle {
        self.border.unwrap_or(fallback)
    }
}