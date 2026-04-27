use base::ui::{
    text::{font::TextFont, style::TextStyleFactory},
    widget::{
        Widget, WidgetId, container::Container, layout::{
            edge_insets::EdgeInsets,
            size::Size,
        }, widgets::{
            label::Label, panel::Panel, row::Row, text_field::TextField
        }
    },
};

use crate::{
    core::app_logic::AppLogic,
    engine::{input::InputState, scene::Scene},
    renderer::{
        Renderer, config::RendererConfig, pass::text::{TextRenderPass, measurer::GlyphonTextMeasurer}
    },
};

pub struct TestApp;

impl TestApp {
    pub fn new() -> Self {
        Self
    }
}

impl AppLogic for TestApp {
    fn create_scene(&self) -> Box<dyn Scene> {
        Box::new(TestScene::new())
    }

    fn create_config(&self) -> RendererConfig {
        RendererConfig {
            clear_color: wgpu::Color { r: 1.0, g: 0.1, b: 0.1, a: 1.0 },
            ..RendererConfig::default()
        }
    }

    fn title(&self) -> &str {
        "Test App"
    }
}

#[derive(Default)]
struct TestScene;

impl TestScene {
    fn new() -> Self {
        Self::default()
    }

    fn build_ui(style: &TextStyleFactory) -> Panel {
        let mut row = Row::new(WidgetId(1)).with_gap(8.0);

        row.push(Box::new(Label::new(
            WidgetId(2),
            "Size",
            style.style(14.0),
        )));

        row.push(Box::new(TextField::new(
            WidgetId(3),
            "24",
            style.style(14.0),
        )));

        row.push(Box::new(Label::new(
            WidgetId(4),
            "mm",
            style.style(14.0),
        )));

        Panel::new(WidgetId(0))
            .with_padding(EdgeInsets::all(8.0))
            .with_child(Box::new(row))
    }
}

impl Scene for TestScene {
    fn update(&mut self, _input: &InputState) {}

    fn build_passes(&self, renderer: &mut Renderer) {
        if renderer.pass_count() == 0 {
            let style = TextStyleFactory::new(TextFont::Regular, [220, 220, 220, 255])
                .with_ratio(1.20);

            let mut panel = Self::build_ui(&style);

            // layout
            let mut measurer = GlyphonTextMeasurer::new();
            let screen = Size {
                w: renderer.width() as f32,
                h: renderer.height() as f32,
            };
            let measured = panel.measure(screen, &mut measurer);
            panel.arrange(
                base::ui::widget::layout::rect::Rect {
                    x: 16.0,
                    y: 16.0,
                    w: measured.w,
                    h: measured.h,
                },
                &mut measurer,
            );

            // collect text
            let mut groups = Vec::new();
            panel.collect_text(&mut groups);

            renderer.add_pass(Box::new(TextRenderPass::new(
                base::ui::text::params::TextParams::new(groups),
            )));
        }
    }
}


// use base::ui::text::{
//     font::TextFont,
//     item::TextItem,
//     params::{TextParam, TextParams},
//     style::TextStyleFactory,
// };

// use crate::{
//     core::app_logic::AppLogic,
//     engine::{input::InputState, scene::Scene},
//     renderer::{config::RendererConfig, pass::text::TextRenderPass, Renderer},
// };

// pub struct TestApp;

// impl TestApp {
//     pub fn new() -> Self {
//         Self
//     }
// }

// impl AppLogic for TestApp {
//     fn create_scene(&self) -> Box<dyn Scene> {
//         Box::new(TestScene::new())
//     }

//     fn create_config(&self) -> RendererConfig {
//         RendererConfig {
//             clear_color: wgpu::Color {
//                 r: 1.0,
//                 g: 0.1,
//                 b: 0.1,
//                 a: 1.0,
//             },
//             ..RendererConfig::default()
//         }
//     }

//     fn title(&self) -> &str {
//         "Test App"
//     }
// }

// #[derive(Default)]
// struct TestScene;

// impl TestScene {
//     fn new() -> Self {
//         Self::default()
//     }
// }

// impl Scene for TestScene {
//     fn update(&mut self, _input: &InputState) {}

//     fn build_passes(&self, renderer: &mut Renderer) {
//         if renderer.pass_count() == 0 {
//             let base = TextStyleFactory::new(TextFont::Regular, [220, 220, 220, 255]).with_ratio(1.20);
//             let header = TextStyleFactory::new(TextFont::Bold, [255, 255, 255, 255]).with_ratio(1.20);
//             let italic = TextStyleFactory::new(TextFont::Italic, [180, 180, 255, 255]).with_ratio(1.20);

//             let groups = vec![
//                 TextParam::new(
//                     header.style(32.0),
//                     vec![TextItem {
//                         text: "Header".into(),
//                         x: 16.0,
//                         y: 20.0,
//                     }],
//                 ),
//                 TextParam::new(
//                     header.style(24.0),
//                     vec![TextItem {
//                         text: "A1".into(),
//                         x: 16.0,
//                         y: 60.0,
//                     }],
//                 ),
//                 TextParam::new(
//                     base.style(14.0),
//                     vec![TextItem {
//                         text: "B1".into(),
//                         x: 120.0,
//                         y: 60.0,
//                     }],
//                 ),
//                 TextParam::new(
//                     italic.style(64.0),
//                     vec![TextItem {
//                         text: "italic note".into(),
//                         x: 16.0,
//                         y: 100.0,
//                     }],
//                 ),
//             ];

//             let params = TextParams::new(groups);
//             renderer.add_pass(Box::new(TextRenderPass::new(params)));
//         }
//     }
// }
