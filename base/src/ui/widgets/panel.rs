use crate::ui::{ 
    box_model::BoxModel, container::WidgetContainer, layout::{layout_params::LayoutParams, rect::Rect, size::Size, text_measurer::TextMeasurer}, macros::{impl_widget_base, impl_widget_container}, text::params::TextParam, widget::{Widget, WidgetBase}
    
};

#[derive(Debug)]
pub struct Panel {
    base: WidgetBase,
    container: WidgetContainer,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            base: WidgetBase::new(),
            container: WidgetContainer::new(),
        }
    }
}

impl_widget_base!(Panel);
impl_widget_container!(Panel);

impl Widget for Panel {
    fn measure(
        &mut self,
        available: Size,
        params: &LayoutParams,
        measurer: &mut dyn TextMeasurer,
    ) -> Size {
        let padding = self.base.box_model().resolved_padding(params.panel_padding);
        let inner = Size {
            w: (available.w - padding.left - padding.right).max(0.0),
            h: (available.h - padding.top - padding.bottom).max(0.0),
        };

        let mut max_w = 0.0f32;
        let mut max_h = 0.0f32;

        self.container.for_each_child_mut(|child| {
            let s = child.measure(inner, params, measurer);
            max_w = max_w.max(s.w);
            max_h = max_h.max(s.h);
        });

        Size {
            w: (max_w + padding.left + padding.right).min(available.w),
            h: (max_h + padding.top + padding.bottom).min(available.h),
        }
    }

    fn arrange(
        &mut self,
        rect: Rect,
        params: &LayoutParams,
        measurer: &mut dyn TextMeasurer,
    ) {
        let mut model = self.base.box_model();
        model.set_rect(rect);
        self.base.set_box_model(model);

        let padding = self.base.box_model().resolved_padding(params.panel_padding);
        let inner = Rect {
            x: rect.x + padding.left,
            y: rect.y + padding.top,
            w: (rect.w - padding.left - padding.right).max(0.0),
            h: (rect.h - padding.top - padding.bottom).max(0.0),
        };

        self.container.for_each_child_mut(|child| {
            child.arrange(inner, params, measurer);
        });
    }

    fn collect_text(&self, out: &mut Vec<TextParam>, params: &LayoutParams) {
        for child in self.container.children() {
            child.collect_text(out, params);
        }
    }

    fn collect_rects(&self, out: &mut Vec<BoxModel>) {
        out.push(self.base.box_model());
        for child in self.container.children() {
            child.collect_rects(out);
        }
    }
}
