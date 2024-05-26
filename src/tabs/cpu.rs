use iced::{
    widget::{row, Column, Scrollable, Space},
    Length,
};

use super::tab_trait::Tab;
use crate::{components::chart::Graph, Message, MsrData};

pub struct Cpu {
    pub msr: MsrData,
    pub graphs_sizes: usize,
    pub cpu_pwr_graph: Graph,
    pub cpu_volt_graph: Graph,
    pub cpu_temp_graph: Graph,
    pub cpu_usage_graph: Graph,
    pub cpu_avg_freq_graph: Graph,
}

impl Tab for Cpu {
    type Message = Message;

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = Column::new()
            // .push(self.generate_sys())
            .push(self.generate_cpu())
            .push(self.cpu_pwr_graph.into_view())
            .push(self.cpu_volt_graph.into_view())
            .push(self.cpu_temp_graph.into_view())
            .push(self.cpu_usage_graph.into_view())
            .push(self.cpu_avg_freq_graph.into_view())
            .spacing(50);

        Scrollable::new(row![content.padding(20), Space::with_width(Length::Fill)]).into()
    }

    fn title(&self) -> String {
        "Cpu".to_owned()
    }

    fn tab_label(&self) -> iced_aw::TabLabel {
        iced_aw::TabLabel::Text(self.title())
    }

    fn content(&self) -> iced::Element<'_, Self::Message> {
        self.view()
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            msr: MsrData::default(),
            graphs_sizes: 50,
            cpu_pwr_graph: Graph::new(50f32, "Cpu Power (W)", 50),
            cpu_volt_graph: Graph::new(1f32, "Cpu Voltage (V)", 50),
            cpu_temp_graph: Graph::new(100f32, "Cpu Temperature (°C)", 50),
            cpu_usage_graph: Graph::new(100f32, "Cpu Usage (%)", 50),
            cpu_avg_freq_graph: Graph::new(2500f32, "Cpu avarage frequency (MHz)", 50),
        }
    }

    pub fn update(&mut self, msr: MsrData) {
        let cores_freq = msr.per_core_freq.clone();

        self.cpu_temp_graph.update(msr.temperature);
        self.cpu_pwr_graph.update(msr.package_power as f32);
        self.cpu_volt_graph.update(msr.voltage as f32);
        self.cpu_usage_graph.update(msr.util as f32);
        self.cpu_avg_freq_graph
            .update((cores_freq.iter().sum::<u64>() / cores_freq.len() as u64) as f32);

        self.msr = msr;
    }

    pub fn resize_graphs(&mut self, size: usize) {
        self.graphs_sizes = size;
        self.cpu_temp_graph.resize(size);
        self.cpu_pwr_graph.resize(size);
        self.cpu_volt_graph.resize(size);
        self.cpu_usage_graph.resize(size);
        self.cpu_avg_freq_graph.resize(size);
    }
}
