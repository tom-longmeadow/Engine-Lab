use crate::ui::widget::layout::{border::BorderStyle, edge_insets::EdgeInsets, rect::Rect};



#[derive(Clone, Copy, Debug, Default)]
pub struct BoxData {
    pub rect: Rect,
    pub padding: EdgeInsets,
    pub background: [u8; 4],
    pub border: BorderStyle,
}