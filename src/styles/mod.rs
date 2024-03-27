use iced::{
    widget::{container, Container},
    Renderer, Theme,
};

use crate::Message;

pub mod boxes;
pub mod title;

pub trait Styled {
    fn padding_style(self, p: u16, s: container::Appearance) -> Self;
}

impl<'a> Styled for Container<'a, Message, Theme, Renderer> {
    fn padding_style(self, p: u16, s: container::Appearance) -> Self {
        self.padding(p).style(s)
    }
}
