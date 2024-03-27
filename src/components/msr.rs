use std::time::{Duration, SystemTime};

use crate::error::AppError;
use crate::misc::prec;
use crate::styles::Styled;
use crate::{styles, App, Message};
use iced::widget::{column, row, text, Column, Container};
use iced::widget::{Row, Text};
use iced::Color;

type ElementSimple<'a> = iced::Element<'a, <App as iced::Application>::Message>;

fn fail_msr<'a>(kind: &str, err: &AppError) -> ElementSimple<'a> {
    text(format!(
        "occured error while requesting MSR({}): {:?}",
        kind, err
    ))
    .into()
}

impl App {
    #[inline]
    pub fn generate_static_cpu<'a>(&mut self) {
        let data = &self.msr;

        self.static_elements.cpu_title = styles::title::title(&data.name);

        let mut cache_vec = vec![];
        for c in &data.cache {
            let title = format!("Cache L{} {:13}", c.level, c.cache_type);
            let title: Text<'static> = text(title).size(21).style(Color::new(0.3, 0.8, 0.3, 1.0));
            let c_data = format!("{} kB", c.size as f64 / 1024.);
            let c_data: Text<'static> = text(c_data).size(21);
            cache_vec.push((title, c_data));
        }
        self.static_elements.cpu_cache = cache_vec;

        self.static_elements.cores_threads = (
            text(format!("Cores: {:4}", data.cores)).size(20),
            text(format!("Threads: {:4}", data.threads)).size(20),
        );
    }

    pub fn generate_cpu<'a>(&self) -> ElementSimple<'a> {
        let data = &self.msr;

        if let Some(err) = &self.state.fails.msr_fail {
            return fail_msr("msr", err);
        }

        let mut cache_column: Column<'a, Message> = column![text("Cache").size(31)];
        for c in &self.static_elements.cpu_cache {
            cache_column = cache_column.push(row![c.0.clone(), c.1.clone()].padding(5).spacing(10));
        }
        let cache_section =
            Container::new(cache_column).padding_style(14, styles::boxes::surround_with_box());

        let len = data.per_core_freq.len() as u64;
        let mut freq_layout: Row<'a, Message> = row![].spacing(20);
        let mut id = 0u64;
        let mut freq = 0u64;

        while id < len {
            let mut col: Column<'a, Message> = column![].spacing(6);
            for _ in 0..6 {
                if id >= len {
                    break;
                }
                freq += data.per_core_freq[id as usize];
                col = col.push(
                    text(format!(
                        "core {}: {}MHz",
                        id, data.per_core_freq[id as usize]
                    ))
                    .size(16),
                );
                id += 1;
            }
            freq_layout = freq_layout.push(col);
        }
        freq = freq / len;

        let freq_section =
            Container::new(column![text("Per Core Frequency").size(31), freq_layout])
                .padding_style(14, styles::boxes::surround_with_box());

        let mut temp_txt = text(format!(
            "Temperature: {: >7}°C",
            prec(data.temperature as f64)
        ))
        .size(20);

        let avg_freq: Text<'a> = text(format!("Avg Frequency: {: >7}MHz", freq)).size(20);
        if data.temperature > 50. {
            temp_txt = temp_txt.style(Color::new(1., 0., 0., 1.));
        };

        let mut usage_txt: Text<'a> = text(format!("Util: {: >7}%", prec(data.util))).size(20);
        if data.util > 50. {
            usage_txt = usage_txt.style(Color::new(1., 0.1, 0.5, 1.));
        };

        let volt: Text<'a> = text(format!("Power: {: >7}W", prec(data.package_power))).size(20);
        let pwr: Text<'a> = text(format!("Voltage: {: >7}V", prec(data.voltage))).size(20);

        let col1 = Column::new()
            .spacing(10)
            .push(self.static_elements.cores_threads.0.clone())
            .push(temp_txt)
            .push(volt);
        let col2 = Column::new()
            .spacing(10)
            .push(self.static_elements.cores_threads.1.clone())
            .push(usage_txt)
            .push(pwr);
        let col3 = Column::new().spacing(10).push(text("")).push(avg_freq);
        let row = row![col1, col2, col3].spacing(35);

        Container::new(
            Column::new()
                .push(self.static_elements.cpu_title.clone())
                .push(row)
                .push(cache_section)
                .push(freq_section)
                .spacing(10),
        )
        .padding_style(14, styles::boxes::surround_with_box())
        .into()
    }

    pub fn generate_sys<'a>(&self) -> ElementSimple<'a> {
        let sys = &self.sys;

        if let Some(err) = &self.state.fails.msr_fail {
            return fail_msr("sys", err);
        }

        let title = styles::title::title(&sys.host_name);

        let system_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let start_time = Duration::from_secs(sys.boot_time);
        let t = (system_time - start_time).as_secs();
        let h = t / 3600;
        let m = (t - (h * 3600)) / 60;
        let since_boot = text(format!("Since boot: {}h, {}m", h, m)).size(20);

        let kernel = text(format!("Kernel: {}", sys.kernel_version.clone())).size(20);
        let os_version = text(sys.os_version.clone()).size(20);

        let row = row![kernel, os_version].spacing(35);

        Container::new(Column::new().push(title).push(since_boot).push(row))
            .padding_style(14, styles::boxes::surround_with_box())
            .into()
    }
}
