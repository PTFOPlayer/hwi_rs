use std::time::{Duration, SystemTime};

use iced::widget::{row, text, Column, Container};
use iced_aw::TabLabel;

use crate::{
    styles::{self, Styled},
    Message, SystemInfo,
};

use super::tab_trait::Tab;

pub struct Sys {
    pub sys: SystemInfo,
}

impl Tab for Sys {
    type Message = Message;

    fn title(&self) -> String {
        "System".to_owned()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let sys = &self.sys;

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

    fn content(&self) -> iced::Element<'_, Self::Message> {
        self.view()
    }
}

impl Sys {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Sys {
    fn default() -> Self {
        Self {
            sys: Default::default(),
        }
    }
}
