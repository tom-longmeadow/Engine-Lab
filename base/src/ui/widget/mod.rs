use crate::ui::{
    text::params::TextParam, 
    widget::{r#box::BoxModel, layout::{
        rect::Rect, size::Size, text_measurer::TextMeasurer
    }}
};


pub mod widgets;
pub mod layout;
pub mod text;
pub mod container;
pub mod r#box;
pub mod macros;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct WidgetId(pub u64);

 
pub trait Widget { 
    fn measure(&mut self, available: Size, measurer: &mut dyn TextMeasurer) -> Size;
    fn arrange(&mut self, rect: Rect, measurer: &mut dyn TextMeasurer); 
    fn collect_text(&self, _out: &mut Vec<TextParam>) {}
    fn collect_rects(&self, _out: &mut Vec<BoxModel>) {}
}


#[derive(Clone, Copy, Debug, Default)]
pub struct WidgetBase {
    id: WidgetId,
    model: BoxModel,
}

impl WidgetBase {
    pub fn new(id: WidgetId) -> Self {
        Self {
            id,
            model: BoxModel::default(),
        }
    }

    pub fn id(&self) -> WidgetId {
        self.id
    }

    pub fn box_model(&self) -> BoxModel {
        self.model
    }

    pub fn set_box_model(&mut self, model: BoxModel) {
        self.model = model;
    }
}
 
 
