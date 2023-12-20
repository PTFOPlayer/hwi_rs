mod misc;
mod statistics;
use error::AppError;
use misc::prec_100;
use statistics::*;
mod error;

use std::time::Duration;

use std::process::Command as sysCommand;

// use main_data::{get_data, MsrData};
use iced::{
    executor,
    theme::Palette,
    widget::{column, container, row, text, Text},
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

struct StaticElements<'a> {
    cpu_title: Text<'a>,
    cpu_cache: Vec<(Text<'a>, Text<'a>)>,
}
impl Default for StaticElements<'_> {
    fn default() -> Self {
        Self {
            cpu_title: text("Unknown"),
            cpu_cache: vec![],
        }
    }
}

struct App {
    tick: u32,
    url: String,
    msr: Result<MsrData, AppError>,
    prep: u8,
    static_elements: StaticElements<'static>,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Msr(Result<MsrData, AppError>),
}

impl Application for App {
    type Message = Message;

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (App, iced::Command<Message>) {
        let app = App::default();
        (app, Command::none())
    }

    fn title(&self) -> String {
        "hwi_rs".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::Tick => self.tick += 1,
            Message::Msr(msr) => {
                if self.prep < 3 {
                    self.msr = msr;
                    self.generate_static_cpu();
                    self.prep += 1;
                } else {
                    self.msr = msr
                }
            }
        }
        if self.tick % 20 == 0 {
            return Command::perform(get_data(self.url.clone()), |x| Message::Msr(x));
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let cpu = self.generate_cpu();
        let content = column![].spacing(10);
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
        let tick = iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick);
        Subscription::batch(vec![tick])
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            tick: Default::default(),
            url: "http://localhost:8000".to_string(),
            msr: Err(AppError::NonInitiated),
            static_elements: StaticElements::default(),
            prep: 0,
        }
    }
}

impl App {
    fn generate_static_cpu<'a>(&mut self) {
        match &self.msr {
            Ok(data) => {
                self.static_elements.cpu_title = {
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

                let mut title_palette = Palette::DARK;
                title_palette.text = Color::new(0.3, 0.9, 0.1, 1.0);

                let mut cache_vec = vec![];
                for c in &data.cache {
                    let title = format!("Cache L{} {}: ", c.level, c.cache_type);
                    let title: Text<'static> = text(title).size(21).style(title_palette.text);
                    let c_data = format!("{} kB", c.size as f64 / 1024.);
                    let c_data: Text<'static> = text(c_data).size(21);
                    cache_vec.push((title, c_data));
                }
                self.static_elements.cpu_cache = cache_vec;
            }
            Err(_) => {}
        };
    }

    fn generate_cpu<'a>(&self) -> impl Into<Element<'a, Message, Renderer>> {
        let msr = &self.msr;
        let data = match msr {
            Ok(ok) => ok,
            Err(AppError::NonInitiated) => return column![row![text("Please wait 10 seconds, preparing data\nIn case of this message displaying more than 30 seconds restart the app")]],
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
        for c in &self.static_elements.cpu_cache {
            cache_section =
                cache_section.push(row![c.0.clone(), c.1.clone()].padding(5).spacing(10));
        }

        let core_thread_info = row![
            text(format!("Cores: {}", data.cores)).size(20),
            text(format!("Threads: {}", data.threads)).size(20)
        ]
        .spacing(20);

        let mut temp_txt = text(format!(
            "Temperature: {:>7}°C",
            prec_100(data.temperature as f64)
        ))
        .size(20);
        if data.temperature > 50. {
            temp_txt = temp_txt.style(Color::new(1., 0., 0., 1.));
        };
        let mut usage_txt = text(format!("Util: {:>7}%", prec_100(data.util))).size(20);
        if data.util > 50. {
            usage_txt = usage_txt.style(Color::new(1., 0.1, 0.5, 1.));
        };
        let freq_temp_usage_info = row![
            text(format!("Frequency: {:>7}Mhz", data.freq)).size(20),
            temp_txt,
            usage_txt
        ]
        .spacing(20);

        let volt = text(format!("Power: {:>7}W", prec_100(data.package_power))).size(20);
        let pwr = text(format!("Voltage: {:>7}V", prec_100(data.voltage))).size(20);
        let voltage_pwr_info = row![volt, pwr].spacing(20);

        return column![
            self.static_elements.cpu_title.clone(),
            core_thread_info,
            freq_temp_usage_info,
            voltage_pwr_info,
            cache_section
        ];
    }
}
