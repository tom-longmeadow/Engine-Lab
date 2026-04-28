use crate::ui::text::style::TextStyle;



 

#[derive(Clone, Debug)]
pub struct WidgetText {
    text: String,
    style: Option<TextStyle>,
}

impl WidgetText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: None,
        }
    }

    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn style(&self) -> Option<TextStyle> {
        self.style
    }

    pub fn set_style(&mut self, style: TextStyle) {
        self.style = Some(style);
    }

    pub fn clear_style(&mut self) {
        self.style = None;
    }

    pub fn resolved_style(&self, fallback: TextStyle) -> TextStyle {
        self.style.unwrap_or(fallback)
    }
}