mod statistics;
use error::AppError;
use statistics::*;
mod error;

use std::arch::x86_64::CpuidResult;
use std::{fmt::format, time::Duration};

use std::process::Command as sysCommand;

// use main_data::{get_data, MsrData};
use iced::{
    executor,
    theme::{Custom, Palette},
    widget::{button, column, container, row, text},
    Application, Color, Command, Element, Renderer, Settings, Subscription, Theme,
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
    msr: Result<MsrData, AppError>,
}

#[derive(Debug, Clone)]
enum Message {
    Click,
    Tick,
    Msr(Result<MsrData, AppError>),
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
            Message::Msr(msr) => self.msr = msr,
        }
        if self.iter2 % 20 == 0 {
            return Command::perform(get_data(self.url.clone()), |x| Message::Msr(x));
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let txt = row![text(format!("hello there, {}, {}", self.iter, self.iter2))];
        let cpu = self.generate_cpu();
        let btn = row![button("click").on_press(Message::Click)];
        let content = column![txt, btn].spacing(10);
        let content = content.push(cpu);
        container(content).padding(10).into()
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
            msr: Err(AppError::NonInitiated),
        }
    }
}

impl App {
    fn generate_cpu<'a>(&self) -> impl Into<Element<'a, Message, Renderer>> {
        let msr = &self.msr;
        let data = match msr {
            Ok(ok) => ok,
            Err(AppError::NonInitiated) => return column![row![text("MSR data not initiated")]],
            Err(err) => {
                return column![row![text(format!(
                    "occured error while requesting MSR: {:?}",
                    err
                ))]]
            }
        };

        let mut title_palette = Palette::DARK;
        title_palette.text = Color::new(0.3, 0.9, 0.1, 1.0);

        let mut cache_section: iced::widget::Column<'a, Message, Renderer> = column![];
        for c in &data.cache {
            let title = format!("Cache L{} {}: ", c.level, c.cache_type);
            let title = text(title).size(21).style(title_palette.text);
            let c_data = format!("{} kB", c.size as f64 / 1024.);
            let c_data = text(c_data).size(21);
            cache_section = cache_section.push(row![title, c_data].padding(5).spacing(10));
        }

        let core_thread_info = row![
            text(format!("Cores: {}", data.cores)).size(20),
            text(format!("Threads: {}", data.threads)).size(20)
        ]
        .spacing(20);

        let freq_temp_info = {
            let mut temp_txt = text(format!("Temperature: {}", data.temperature.round())).size(20);
            if data.temperature > 50. {
                temp_txt = temp_txt.style(Color::new(1., 0.,0.,1.));
            };
            row![text(format!("Frequency: {}", data.freq)).size(20), temp_txt].spacing(20)
        };
        let cpu_title = {
            if data.name.contains("Intel") {
                text(format!("{}", data.name))
                    .size(35)
                    .style(Color::new(0.1, 0.2, 0.9, 1.))
            } else if data.name.contains("AMD") {
                text(format!("{}", data.name))
                    .size(35)
                    .style(Color::new(0.9, 0.2, 0.3, 1.))
            } else {
                text(format!("{}", data.name)).size(35)
            }
        };

        return column![
            cpu_title,
            core_thread_info,
            freq_temp_info,
            row![cache_section]
        ];
    }
}
