mod components;
mod misc;
mod state;
mod statistics;
use error::AppError;
use state::{AxisState, GpuState, State, StaticElements};
use statistics::*;
mod error;

use std::time::Duration;

use std::process::Command as sysCommand;

use iced::{
    executor,
    widget::{button, checkbox, column, row, text_input, Column, Scrollable, Space},
    Application, Command, Length, Settings, Subscription, Theme,
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
    state: State,
    url: String,
    msr: MsrData,
    sys: SystemInfo,
    static_elements: StaticElements<'static>,
    axis_state: AxisState,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Msr(MsrData),
    Prep {
        msr: Result<MsrData, AppError>,
        sys: Result<SystemInfo, AppError>,
    },
    Fail(AppError),
    CheckboxMsg {
        state: bool,
    },
    SplitSwitch,
    Url(String),
    Resize(u16),
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

                if self.generate_radeon().is_err() {
                    self.state.gpu = GpuState::None
                } else {
                    self.state.gpu = GpuState::Radeon
                }
            }
            Message::Tick => {
                return Command::perform(get_data(self.url.clone()), |x| match x {
                    Ok(res) => Message::Msr(res),
                    Err(err) => Message::Fail(err),
                });
            }
            Message::Msr(msr) => {
                let state = &mut self.state;
                if state.graphs_switch {
                    state.cpu_temp_graph.modify_graph(msr.temperature);
                    state.cpu_pwr_graph.modify_graph(msr.package_power as f32);
                    state.cpu_usage_graph.modify_graph(msr.util as f32);
                    state.cpu_avg_freq_graph.modify_graph(
                        (msr.per_core_freq.iter().sum::<u64>() / msr.per_core_freq.len() as u64)
                            as f32,
                    );
                }
                self.msr = msr;
            }
            Message::Fail(fail) => self.state.fails.msr_fail = Some(fail),
            Message::CheckboxMsg { state } => {
                self.state.graphs_switch = state;
            }
            Message::Url(url) => {
                self.url = url;
            }
            Message::Resize(x) => self.axis_state.set_divider(x),
            Message::SplitSwitch => self.axis_state.switch(),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let graphs_switch = checkbox("graphs switch", self.state.graphs_switch)
            .on_toggle(|x| Message::CheckboxMsg { state: x });

        let split_switch = button("toggle axis").on_press(Message::SplitSwitch);
        let url_input = text_input(&self.url, &self.url)
            .on_input(|url| Message::Url(url))
            .width(350.);

        let misc_row = row![graphs_switch, split_switch, url_input]
            .padding(20)
            .spacing(20);

        let content = Column::new()
            .push(self.generate_sys())
            .push(self.generate_cpu())
            .push(match self.state.gpu {
                GpuState::None => column![].into(),
                GpuState::Radeon => self.generate_radeon().unwrap_or(column![].into()),
                GpuState::Nvidia => column![].into(),
            })
            .spacing(50);

        let mut graphs = column![].spacing(50);

        if self.state.graphs_switch {
            graphs = graphs
                .push(self.state.cpu_pwr_graph.into_view())
                .push(self.state.cpu_temp_graph.into_view())
                .push(self.state.cpu_usage_graph.into_view())
                .push(self.state.cpu_avg_freq_graph.into_view());
        }

        let scroll = iced_aw::Split::new(
            Scrollable::new(row![content.padding(20), Space::with_width(Length::Fill)]),
            Scrollable::new(row![graphs.padding(20), Space::with_width(Length::Fill)]),
            self.axis_state.divider,
            self.axis_state.split_axis,
            Message::Resize,
        );

        let final_element = column![misc_row, scroll].width(Length::Fill);

        final_element.into()
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
            url: "http://localhost:7172".to_string(),
            msr: MsrData::default(),
            sys: SystemInfo::default(),
            static_elements: StaticElements::default(),
            axis_state: AxisState::default(),
        }
    }
}

async fn prepare(url: String) -> (Result<MsrData, AppError>, Result<SystemInfo, AppError>) {
    (get_data(url.clone()).await, get_system_data(url).await)
}
