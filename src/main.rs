mod components;
mod misc;
mod state;
mod statistics;
use error::AppError;
use plotters_iced::ChartWidget;
use state::{GpuState, State};
use statistics::*;
mod error;

use std::time::Duration;

use std::process::Command as sysCommand;

// use main_data::{get_data, MsrData};
use iced::{
    executor,
    widget::{column, container, text, Text},
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

struct StaticElements<'a> {
    cpu_title: Text<'a>,
    cpu_cache: Vec<(Text<'a>, Text<'a>)>,
    cores_threads: (Text<'a>, Text<'a>),
}

impl<'a> Default for StaticElements<'a> {
    fn default() -> Self {
        Self {
            cpu_title: text("Unknown"),
            cpu_cache: vec![],
            cores_threads: (text(""), text("")),
        }
    }
}

struct App {
    state: State,
    url: String,
    msr: MsrData,
    sys: SystemInfo,
    static_elements: StaticElements<'static>,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Msr(MsrData),
    Prep {
        msr: Result<MsrData, AppError>,
        sys: Result<SystemInfo, AppError>,
    },
    Fail(AppError),
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (App, iced::Command<Message>) {
        let app = App::default();
        let url = app.url.clone();
        (
            app,
            Command::perform(prepare(url), |x| {
                return Message::Prep { msr: x.0, sys: x.1 };
            }),
        )
    }

    fn title(&self) -> String {
        "hwi_rs".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::Prep { msr, sys } => {
                match msr {
                    Ok(res) => self.msr = res,
                    Err(err) => self.state.fails.msr_fail = Some(err),
                }
                match sys {
                    Ok(res) => self.sys = res,
                    Err(err) => self.state.fails.sys_fail = Some(err),
                }
                self.generate_static_cpu();
            }
            Message::Tick => {
                return Command::perform(get_data(self.url.clone()), |x| match x {
                    Ok(res) => Message::Msr(res),
                    Err(err) => Message::Fail(err),
                });
            }
            Message::Msr(msr) => {
                self.state.cpu_temp_graph.modify_graph(msr.temperature);
                self.state.cpu_pwr_graph.modify_graph(msr.package_power);
                self.msr = msr;
            }
            Message::Fail(fail) => self.state.fails.msr_fail = Some(fail),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let cpu = self.generate_cpu();
        let sys = self.generate_sys();
        let cpu_pwr =
            ChartWidget::new(self.state.cpu_pwr_graph.clone()).height(iced::Length::Fixed(250.));
        let cpu_temp =
            ChartWidget::new(self.state.cpu_temp_graph.clone()).height(iced::Length::Fixed(250.));
        let content = column![sys, cpu, cpu_pwr, cpu_temp].spacing(50);

        container(content).padding(10).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subs = vec![];
        let tick = iced::time::every(Duration::from_secs_f64(2.)).map(|_| Message::Tick);
        subs.push(tick);
        match self.state.gpu {
            GpuState::Nvidia => {}
            GpuState::Radeon => {}
            GpuState::Intel => {}
            _ => {}
        };
        Subscription::batch(subs)
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: State::default(),
            url: "http://localhost:8000".to_string(),
            msr: MsrData::default(),
            sys: SystemInfo::default(),
            static_elements: StaticElements::default(),
        }
    }
}

async fn prepare(url: String) -> (Result<MsrData, AppError>, Result<SystemInfo, AppError>) {
    (get_data(url.clone()).await, get_system_data(url).await)
}
