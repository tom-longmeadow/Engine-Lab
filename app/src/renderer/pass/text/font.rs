use base::ui::text::font::{FontStyle, TextFont};

use glyphon::cosmic_text;

pub trait TextFontExt {
    fn attrs(&self) -> glyphon::Attrs<'_>;
}

impl TextFontExt for TextFont {
    fn attrs(&self) -> glyphon::Attrs<'_> {
        let weight = cosmic_text::Weight(self.weight().0);
        let style = match self.font_style() {
            FontStyle::Italic => cosmic_text::Style::Italic,
            FontStyle::Normal => cosmic_text::Style::Normal,
        };

        glyphon::Attrs::new()
            .family(cosmic_text::Family::Name(self.family_name()))
            .weight(weight)
            .style(style)
    }
}
