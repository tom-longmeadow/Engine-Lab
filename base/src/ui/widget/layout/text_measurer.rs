use crate::ui::{
    text::style::TextStyle, 
    widget::layout::size::Size
};
 

pub trait TextMeasurer {
    fn measure(&mut self, text: &str, style: &TextStyle) -> Size;
}