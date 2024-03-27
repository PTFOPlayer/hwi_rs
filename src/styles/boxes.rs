use iced::{Border, Color, Shadow};

pub fn surround_with_box() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        border: Border {
            color: Color::from_rgba8(10, 10, 10, 1.),
            width: 3.,
            radius: [12., 12., 12., 12.].into(),
        },
        text_color: None,
        background: None,
        shadow: Shadow::default(),
    }
}