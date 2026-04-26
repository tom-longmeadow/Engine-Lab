use glyphon::cosmic_text;

#[derive(Clone, Debug)]
pub enum TextFont {
    Regular,
    Bold,
    Italic,
    BoldItalic,
    Light,
    LightItalic,
    Thin,
    ThinItalic,
    Medium,
    MediumItalic,
    SemiBold,
    SemiBoldItalic,
    ExtraBold,
    ExtraBoldItalic,
}

 macro_rules! font {
    ($name:literal) => {
        include_bytes!(concat!(
            "../../../../assets/fonts/JetBrainsMono-2/fonts/ttf/",
            $name
        ))
    };
}

impl TextFont {
    pub fn all() -> &'static [TextFont] {
        &[
            TextFont::Regular,
            TextFont::Bold,
            TextFont::Italic,
            TextFont::BoldItalic,
            TextFont::Light,
            TextFont::LightItalic,
            TextFont::Thin,
            TextFont::ThinItalic,
            TextFont::Medium,
            TextFont::MediumItalic,
            TextFont::SemiBold,
            TextFont::SemiBoldItalic,
            TextFont::ExtraBold,
            TextFont::ExtraBoldItalic,
        ]
    }

   

    pub fn font_bytes(&self) -> &'static [u8] {
        match self {
            TextFont::Regular         => font!("JetBrainsMono-Regular.ttf"),
            TextFont::Bold            => font!("JetBrainsMono-Bold.ttf"),
            TextFont::Italic          => font!("JetBrainsMono-Italic.ttf"),
            TextFont::BoldItalic      => font!("JetBrainsMono-BoldItalic.ttf"),
            TextFont::Light           => font!("JetBrainsMono-Light.ttf"),
            TextFont::LightItalic     => font!("JetBrainsMono-LightItalic.ttf"),
            TextFont::Thin            => font!("JetBrainsMono-Thin.ttf"),
            TextFont::ThinItalic      => font!("JetBrainsMono-ThinItalic.ttf"),
            TextFont::Medium          => font!("JetBrainsMono-Medium.ttf"),
            TextFont::MediumItalic    => font!("JetBrainsMono-MediumItalic.ttf"),
            TextFont::SemiBold        => font!("JetBrainsMono-SemiBold.ttf"),
            TextFont::SemiBoldItalic  => font!("JetBrainsMono-SemiBoldItalic.ttf"),
            TextFont::ExtraBold       => font!("JetBrainsMono-ExtraBold.ttf"),
            TextFont::ExtraBoldItalic => font!("JetBrainsMono-ExtraBoldItalic.ttf"),
        }
    }

    pub fn family_name(&self) -> &'static str {
        "JetBrains Mono"
    }

    pub fn weight(&self) -> cosmic_text::Weight {
        match self {
            TextFont::Thin | TextFont::ThinItalic           => cosmic_text::Weight(100),
            TextFont::Light | TextFont::LightItalic         => cosmic_text::Weight(300),
            TextFont::Regular | TextFont::Italic            => cosmic_text::Weight::NORMAL,
            TextFont::Medium | TextFont::MediumItalic       => cosmic_text::Weight(500),
            TextFont::SemiBold | TextFont::SemiBoldItalic   => cosmic_text::Weight(600),
            TextFont::Bold | TextFont::BoldItalic           => cosmic_text::Weight::BOLD,
            TextFont::ExtraBold | TextFont::ExtraBoldItalic => cosmic_text::Weight(800),
        }
    }

    pub fn style(&self) -> cosmic_text::Style {
        match self {
            TextFont::Italic
            | TextFont::BoldItalic
            | TextFont::LightItalic
            | TextFont::ThinItalic
            | TextFont::MediumItalic
            | TextFont::SemiBoldItalic
            | TextFont::ExtraBoldItalic => cosmic_text::Style::Italic,
            _ => cosmic_text::Style::Normal,
        }
    }

    pub fn attrs(&self) -> glyphon::Attrs<'_> {
        glyphon::Attrs::new()
            .family(cosmic_text::Family::Name(self.family_name()))
            .weight(self.weight())
            .style(self.style())
    }
}