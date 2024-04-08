mod components;
mod misc;
mod state;
mod statistics;
use error::AppError;
use iced_aw::Tabs;
use state::{AxisState, GpuState, State};
use statistics::*;
use tabs::tab_trait::Tab;
mod error;
mod styles;
mod tabs;

use std::time::Duration;

use std::process::Command as sysCommand;

use iced::{executor, Application, Command, Settings, Subscription, Theme};

fn main() {
    let mut msr = sysCommand::new("systemctl");
    msr.args(["start", "msr_server.service"]);
    _ = msr.output();

    let mut settings = Settings::default();
    settings.window.transparent = true;

    let _ = App::run(settings);
}

struct App {
    state: State,
    page: Page,
    axis_state: AxisState,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Null,
    Msr(MsrData),
    Prep {
        msr: Result<MsrData, AppError>,
        sys: Result<SystemInfo, AppError>,
    },
    CheckboxMsg {
        state: bool,
    },
    SplitSwitch,
    Url(String),
    SplitResize(u16),
    ResizeGraphs(usize),
    SetPage(Page),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Page {
    #[default]
    Sys,
    CPU,
    GPU,
    Settings,
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (App, iced::Command<Message>) {
        let app = App::default();
        let url = app.state.settings.url.clone();
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
                    Ok(res) => self.state.cpu.msr = res,
                    Err(_err) => {}
                }
                match sys {
                    Ok(res) => self.state.sys.sys = res,
                    Err(_err) => {}
                }

                if self.generate_radeon().is_err() {
                    self.state.gpu = GpuState::None
                } else {
                    self.state.gpu = GpuState::Radeon
                }
            }
            Message::Tick => {
                let url = self.state.settings.url.clone();

                return Command::perform(get_data(url), |x| match x {
                    Ok(res) => Message::Msr(res),
                    _ => Message::Null,
                });
            }
            Message::Msr(msr) => {
                self.state.cpu.update(msr);
            }
            Message::CheckboxMsg { state } => {
                self.state.settings.graphs_switch = state;
            }
            Message::Url(url) => {
                self.state.settings.url = url;
            }
            Message::SplitResize(x) => self.axis_state.set_divider(x),
            Message::SplitSwitch => self.axis_state.switch(),
            Message::ResizeGraphs(x) => {
                self.state.cpu.resize_graphs(x);
                self.state.settings.graphs_sizes = x
            }
            Message::SetPage(page) => self.page = page,
            Message::Null => {}
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // init radeon
        // .push(match self.state.gpu {
        //     GpuState::None => column![].into(),
        //     GpuState::Radeon => self.generate_radeon().unwrap_or(column![].into()),
        //     GpuState::Nvidia => column![].into(),
        // })
        let tab = Tabs::new(Message::SetPage)
            .push(Page::CPU, self.state.cpu.tab_label(), self.state.cpu.view())
            .push(Page::Sys, self.state.sys.tab_label(), self.state.sys.view())
            .push(
                Page::Settings,
                self.state.settings.tab_label(),
                self.state.settings.view(),
            )
            .set_active_tab(&self.page);

        tab.into()
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
            _ => {}
        };
        Subscription::batch(subs)
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: State::default(),
            axis_state: AxisState::default(),
            page: Page::default(),
        }
    }
}

async fn prepare(url: String) -> (Result<MsrData, AppError>, Result<SystemInfo, AppError>) {
    (get_data(url.clone()).await, get_system_data(url).await)
}
