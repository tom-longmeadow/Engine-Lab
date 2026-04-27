use base::ui::{
    text::{font::TextFont, params::TextParams, style::TextStyleFactory},
    widget::{
        Widget,
        layout::{
            color::Color,
            edge_insets::EdgeInsets,
            layout_params::LayoutParams,
            rect::Rect,
            size::Size,
        },
        widgets::{column::Column, grid::Grid, label::Label, panel::Panel, row::Row, text_field::TextField},
    },
};

use crate::{
    engine::{input::InputState, scene::Scene},
    renderer::{
        pass::text::{measurer::GlyphonTextMeasurer, TextRenderPass},
        Renderer,
    },
};

#[derive(Default)]
pub struct TestScene;

impl TestScene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_ui(heading_style: base::ui::text::style::TextStyle) -> Panel {
         // Row 1: app/header info
        let mut header_row = Row::new()
            .with_gap(20.0)
            .with_background(Color::rgb(40, 44, 52));

        header_row.push(Box::new(Label::new("Model Lab").with_style(heading_style)));
       

        // Row 2: primary editable fields
        let mut input_row = Row::new();
        input_row.push(Box::new(Label::new("Name:")));
        input_row.push(Box::new(TextField::new("Engine Bolt"))); 

        // Grid: detailed properties (2 columns = label/value)
        let mut details_grid = Grid::new(2).with_background(Color::rgb(28, 31, 38));
        details_grid.push(Box::new(Label::new("Mass")));
        details_grid.push(Box::new(TextField::new("1.42 kg")));
        details_grid.push(Box::new(Label::new("Tolerance")));
        details_grid.push(Box::new(TextField::new("±0.05 mm")));
        details_grid.push(Box::new(Label::new("Batch")));
        details_grid.push(Box::new(TextField::new("A-104")));
        details_grid.push(Box::new(Label::new("Process")));
        details_grid.push(Box::new(TextField::new("CNC")));
        details_grid.push(Box::new(Label::new("Supplier")));
        details_grid.push(Box::new(TextField::new("Acme Industrial")));

        // Column containing exactly two rows and one grid
        let mut root_col = Column::new();
        root_col.push(Box::new(header_row));
        root_col.push(Box::new(input_row));
        root_col.push(Box::new(details_grid));

        // Top-level panel contains the column
        Panel::new()
            .with_padding(EdgeInsets::all(16.0))
            .with_background(Color::rgb(20, 22, 26))
            .with_child(Box::new(root_col))
    }
}

impl Scene for TestScene {
    fn update(&mut self, _input: &InputState) {}

    fn build_passes(&self, renderer: &mut Renderer) {
        if renderer.pass_count() == 0 {
            let style_factory =
                TextStyleFactory::new(TextFont::Regular, [220, 220, 220, 255]).with_ratio(1.20);

            let body_style = style_factory.style(34.0);
            let heading_style = style_factory.style(62.0);

            // Global defaults (used by almost everything)
            let params = LayoutParams::default()
                .with_text(body_style)
                .with_gap(8.0)
                .with_control_padding(EdgeInsets::all(6.0))
                .with_panel_padding(EdgeInsets::all(10.0));

            let mut panel = Self::build_ui(heading_style);

            let mut measurer = GlyphonTextMeasurer::new();
            let screen = Size {
                w: renderer.width() as f32,
                h: renderer.height() as f32,
            };

            let measured = panel.measure(screen, &params, &mut measurer);
            panel.arrange(
                Rect {
                    x: 16.0,
                    y: 16.0,
                    w: measured.w,
                    h: measured.h,
                },
                &params,
                &mut measurer,
            );

            let mut groups = Vec::new();
            panel.collect_text(&mut groups, &params);

            renderer.add_pass(Box::new(TextRenderPass::new(TextParams::new(groups))));
        }
    }
}