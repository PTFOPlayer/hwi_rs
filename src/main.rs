mod statistics;
use statistics::*;
mod error;

use std::time::Duration;

use std::process::Command as sysCommand;

// use main_data::{get_data, MsrData};
use iced::{
    executor,
    widget::{button, column, container, row, text},
    Application, Command, Settings, Subscription, Theme,
};

fn main() {
    let mut msr = sysCommand::new("systemctl");
    msr.args(["start", "msr_server.service"]);
    _ = msr.output();

    let mut settings = Settings::default();
    settings.window.transparent = true;
    let _ = App::run(settings);
}

struct App {
    iter: u32,
    iter2: u32,
    url: String,
}

#[derive(Debug, Clone)]
enum Message {
    Click,
    Tick,
    Msr(MsrData),
}

impl Application for App {
    type Message = Message;

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (App, iced::Command<Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String {
        "hwi_rs".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::Click => self.iter += 10,
            Message::Tick => self.iter2 += 1,
            Message::Msr(msr) => println!("{:?}", msr),
        }
        if self.iter2 % 20 == 0 {
            return Command::perform(get_data(self.url.clone()), |x| Message::Msr(x.unwrap()));
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let txt = row![text(format!("hello there, {}, {}", self.iter, self.iter2))];
        let btn = row![button("click").on_press(Message::Click)];
        container(column![txt, btn].spacing(10)).padding(10).into()
    }

    fn theme(&self) -> iced::Theme {
        let theme = iced::Theme::Dark;
        theme
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let tick = iced::time::every(Duration::from_millis((self.iter + 100).into()))
            .map(|_| Message::Tick);
        Subscription::batch(vec![tick])
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            iter: Default::default(),
            iter2: Default::default(),
            url: "http://localhost:8000".to_string(),
        }
    }
}
