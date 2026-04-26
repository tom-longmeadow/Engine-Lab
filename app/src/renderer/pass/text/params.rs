use crate::renderer::pass::text::{item::TextItem, style::TextStyle};


pub struct TextParams {
    pub default_style: TextStyle,
    pub items: Vec<TextItem>,
}

impl Default for TextParams {
    fn default() -> Self {
        Self {
            default_style: TextStyle::default(),
            items: vec![TextItem {
                text: "Hello".into(),
                x: 20.0,
                y: 20.0,
                style: None,
            }],
        }
    }
}
