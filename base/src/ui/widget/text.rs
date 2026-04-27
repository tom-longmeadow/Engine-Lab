

use crate::ui::{
    text::{ 
        style::TextStyle
    }, 
    widget::{ 
        layout::{ 
            size::Size, 
            text_measurer::TextMeasurer
        }
    },
};


#[derive(Clone, Debug)]
pub struct WidgetText { 
    text: String,
    style: TextStyle,
}

impl WidgetText {
    pub fn new(text: impl Into<String>, style: TextStyle) -> Self {
        Self { 
            text: text.into(),
            style,
        }
    }

    pub fn measure_clamped(&self, available: Size, measurer: &mut dyn TextMeasurer) -> Size {
        let s = measurer.measure(&self.text, &self.style);
        Size {
            w: s.w.min(available.w),
            h: s.h.min(available.h),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn style(&self) -> TextStyle {
        self.style
    }

    pub fn set_style(&mut self, style: TextStyle) {
        self.style = style;
    }
 
}