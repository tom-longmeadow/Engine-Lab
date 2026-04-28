use crate::ui::{
    box_model::BoxModel, layout::{
            layout_params::LayoutParams,
            rect::Rect,
            size::Size,
            text_measurer::TextMeasurer,
    }, macros::{impl_widget_base, impl_widget_text}, text::{item::TextItem, params::TextParam}, widget::{Widget, WidgetBase}, widget_text::WidgetText 
     
};

#[derive(Clone, Debug)]
pub struct TextField {
    base: WidgetBase,
    text: WidgetText,
    placeholder: String,
}

impl TextField {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            base: WidgetBase::new(),
            text: WidgetText::new(value),
            placeholder: String::new(),
        }
    }

    pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
        self.placeholder = placeholder.into();
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    fn display_text(&self) -> &str {
        if self.text.text().is_empty() {
            &self.placeholder
        } else {
            self.text.text()
        }
    }
}

impl_widget_base!(TextField);
impl_widget_text!(TextField);

impl Widget for TextField {
    fn measure(
        &mut self,
        available: Size,
        params: &LayoutParams,
        measurer: &mut dyn TextMeasurer,
    ) -> Size {
        let style = self.text.resolved_style(params.text);
        let text = if self.text.text().is_empty() && !self.placeholder.is_empty() {
            &self.placeholder
        } else {
            self.text.text()
        };

        let s = measurer.measure(text, &style);
        Size {
            w: s.w.min(available.w),
            h: s.h.min(available.h),
        }
    }

    fn arrange(
        &mut self,
        rect: Rect,
        _params: &LayoutParams,
        _measurer: &mut dyn TextMeasurer,
    ) {
        let mut model = self.base.box_model();
        model.set_rect(rect);
        self.base.set_box_model(model);
    }

   fn collect_text(&self, out: &mut Vec<TextParam>, params: &LayoutParams) {
        let rect = self.base.box_model().rect();
        let style = self.text.resolved_style(params.text);
        out.push(TextParam::new(
            style,
            vec![TextItem {
                text: self.display_text().to_string(),
                x: rect.x,
                y: rect.y,
            }],
        ));
    }

    fn collect_rects(&self, out: &mut Vec<BoxModel>) {
        out.push(self.base.box_model());
    }
}