

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

pub struct TextWidgetData { 
    pub text: String,
    pub style: TextStyle,
}

impl TextWidgetData {
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
 
}