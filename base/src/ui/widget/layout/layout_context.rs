use crate::ui::widget::layout::{layout_params::LayoutParams, text_measurer::TextMeasurer};


pub struct LayoutContext<'a> {
    pub text_measurer: &'a mut dyn TextMeasurer,
    pub params: &'a LayoutParams,
}