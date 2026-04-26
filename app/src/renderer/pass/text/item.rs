use crate::renderer::pass::text::style::TextStyle;



pub struct TextItem {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub style: Option<TextStyle>, // None => use default_style
}