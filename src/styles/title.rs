use iced::{
    widget::{text, Text},
    Color, Renderer, Theme,
};

const TITLE_SIZE: u16 = 35;

const INTEL_COLOR: Color = Color {
    r: 0.0,
    g: 0.75686276,
    b: 0.9529412,
    a: 1.0,
};

const AMD_COLOR: Color = Color {
    r: 0.92941177,
    g: 0.10980392,
    b: 0.14117648,
    a: 1.0,
};

pub fn title<'a>(s: &str) -> Text<'a, Theme, Renderer> {
    if s.contains("Intel") {
        text(format!("{}", s)).size(TITLE_SIZE).style(INTEL_COLOR)
    } else if s.contains("AMD") || s.contains("Radeon") || s.contains("Navi") {
        text(format!("{}", s)).size(TITLE_SIZE).style(AMD_COLOR)
    } else {
        text(format!("{}", s)).size(TITLE_SIZE)
    }
}
