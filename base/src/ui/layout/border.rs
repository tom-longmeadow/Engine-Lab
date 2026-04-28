use crate::ui::layout::color::Color;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BorderStyle {
    pub color: Color,
    pub width: f32,
    pub radius: f32,
}

impl BorderStyle {
    pub fn new(color: Color, width: f32, radius: f32) -> Self {
        Self { color, width, radius }
    }
}