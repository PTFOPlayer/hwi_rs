use iced::widget::{checkbox, column, text_input};
use iced_aw::number_input;

use crate::Message;

use super::tab_trait::Tab;

pub struct Settings {
    pub url: String,
    pub graphs_sizes: usize,
    pub graphs_switch: bool,
}

impl Tab for Settings {
    type Message = Message;

    fn title(&self) -> String {
        "Settings".to_owned()
    }

    fn tab_label(&self) -> iced_aw::TabLabel {
        iced_aw::TabLabel::Text(self.title())
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let graphs_switch = checkbox("graphs switch", self.graphs_switch)
            .on_toggle(|x| Message::CheckboxMsg { state: x });

        let url_input = text_input(&self.url, &self.url)
            .on_input(|url| Message::Url(url))
            .width(350.);
        let graphs_size_input = number_input(self.graphs_sizes, 1000usize, |size| {
            Message::ResizeGraphs(size)
        });

        let misc_column = column![graphs_switch, url_input, graphs_size_input]
            .padding(20)
            .spacing(20);

        misc_column.into()
    }

    fn content(&self) -> iced::Element<'_, Self::Message> {
        self.view()
    }
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            url: "http://localhost:7172".to_string(),
            graphs_sizes: 50usize,
            graphs_switch: false,
        }
    }
}
