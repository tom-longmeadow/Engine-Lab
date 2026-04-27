use crate::ui::widget::layout::{border::BorderStyle, edge_insets::EdgeInsets, rect::Rect};

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxModel {
    rect: Rect,
    padding: EdgeInsets,
    background: [u8; 4],
    border: BorderStyle,
}

impl BoxModel {

    pub fn new(rect: Rect, padding: EdgeInsets, background: [u8; 4], border: BorderStyle) -> Self {
        Self {
            rect,
            padding,
            background,
            border,
        }
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn padding(&self) -> EdgeInsets {
        self.padding
    }

    pub fn set_padding(&mut self, padding: EdgeInsets) {
        self.padding = padding;
    }

    pub fn background(&self) -> [u8; 4] {
        self.background
    }

    pub fn set_background(&mut self, background: [u8; 4]) {
        self.background = background;
    }

    pub fn border(&self) -> BorderStyle {
        self.border
    }

    pub fn set_border(&mut self, border: BorderStyle) {
        self.border = border;
    }
}