use std::time::{Duration, Instant, SystemTime};

use crate::error::AppError;
use crate::misc::prec;
use crate::{App, Message};
use iced::widget::Text;
use iced::widget::{column, row, text, Column};
use iced::Color;

impl App {
    #[inline]
    pub fn generate_static_cpu<'a>(&mut self) {
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

                let mut cache_vec = vec![];
                for c in &data.cache {
                    let title = format!("Cache L{} {}: ", c.level, c.cache_type);
                    let title: Text<'static> =
                        text(title).size(21).style(Color::new(0.3, 0.8, 0.3, 1.0));
                    let c_data = format!("{} kB", c.size as f64 / 1024.);
                    let c_data: Text<'static> = text(c_data).size(21);
                    cache_vec.push((title, c_data));
                }
                self.static_elements.cpu_cache = cache_vec;

                self.static_elements.cores_threads = (
                    text(format!("Cores: {}", data.cores)).size(20),
                    text(format!("Threads: {}", data.threads)).size(20),
                );
            }
            Err(_) => {}
        };
    }

    pub fn generate_cpu<'a>(&self) -> Column<'a, Message> {
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

        let mut cache_section: Column<'a, Message> = column![];
        for c in &self.static_elements.cpu_cache {
            cache_section =
                cache_section.push(row![c.0.clone(), c.1.clone()].padding(5).spacing(10));
        }

        let mut temp_txt = text(format!(
            "Temperature: {:>7}°C",
            prec(data.temperature as f64)
        ))
        .size(20);

        let avg_freq: Text<'a> = text(format!("Avg Frequency: {}", data.freq)).size(20);
        if data.temperature > 50. {
            temp_txt = temp_txt.style(Color::new(1., 0., 0., 1.));
        };

        let mut usage_txt: Text<'a> = text(format!("Util: {:>7}%", prec(data.util))).size(20);
        if data.util > 50. {
            usage_txt = usage_txt.style(Color::new(1., 0.1, 0.5, 1.));
        };

        let volt: Text<'a> = text(format!("Power: {:>7}W", prec(data.package_power))).size(20);
        let pwr: Text<'a> = text(format!("Voltage: {:>7}V", prec(data.voltage))).size(20);

        let col1 = column![self.static_elements.cores_threads.0.clone(), temp_txt, volt];
        let col2 = column![self.static_elements.cores_threads.1.clone(), usage_txt, pwr];
        let col3 = column![text(""), avg_freq];
        let row = row![col1, col2, col3].spacing(35);
        return column![self.static_elements.cpu_title.clone(), row, cache_section];
    }

    pub fn generate_sys<'a>(&self) -> Column<'a, Message> {
        let sys = match &self.sys {
            Ok(res) => res,
            Err(err) => {
                println!("{:?}", err);
                return column![];
            }
        };

        let mut host_name = sys.host_name.clone();
        if host_name.contains(".home") {
            host_name = host_name.strip_suffix(".home").unwrap().to_owned();
        }

        let title = text(host_name);
        let system_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let start_time = Duration::from_secs(sys.boot_time);
        let t = (system_time - start_time).as_secs();
        let h = t / 3600;
        let m = (t - (h * 3600)) / 60;
        let since_boot = text(format!("since boot: {}h, {}m", h, m));
        return column![title, since_boot];
    }
}
