use iced::Element;
use iced_aw::TabLabel;

pub trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message>;

    fn content(&self) -> Element<'_, Self::Message>;
}
