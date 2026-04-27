use crate::ui::widget::layout::edge_insets::EdgeInsets;


#[derive(Clone, Copy, Debug)]
pub struct LayoutParams {
    pub control_padding: EdgeInsets,
    pub panel_padding: EdgeInsets,
    pub gap: f32,
}

impl Default for LayoutParams {
    fn default() -> Self {
        Self {
            control_padding: EdgeInsets::all(6.0),
            panel_padding: EdgeInsets::all(8.0),
            gap: 6.0,
        }
    }
}