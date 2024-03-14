use iced::widget::{text, Text};

use crate::{components::chart::Graph, error::AppError, MsrData};

pub struct State {
    pub fails: Fails,
    pub gpu: GpuState,
    pub graphs_switch: bool,
    pub graphs_sizes: usize,
    pub cpu_pwr_graph: Graph,
    pub cpu_volt_graph: Graph,
    pub cpu_temp_graph: Graph,
    pub cpu_usage_graph: Graph,
    pub cpu_avg_freq_graph: Graph,
}

impl Default for State {
    fn default() -> Self {
        Self {
            fails: Fails::default(),
            gpu: GpuState::None,
            graphs_switch: false,
            graphs_sizes: 50,
            cpu_pwr_graph: Graph::new(50f32, "Cpu Power (W)", 50),
            cpu_volt_graph: Graph::new(1f32, "Cpu Voltage (V)", 50),
            cpu_temp_graph: Graph::new(100f32, "Cpu Temperature (°C)", 50),
            cpu_usage_graph: Graph::new(100f32, "Cpu Usage (%)", 50),
            cpu_avg_freq_graph: Graph::new(2500f32, "Cpu avarage frequency (MHz)", 50),
        }
    }
}

impl State {
    pub fn update_graphs(&mut self, msr: &MsrData) {
        self.cpu_temp_graph.update(msr.temperature);
        self.cpu_pwr_graph.update(msr.package_power as f32);
        self.cpu_volt_graph.update(msr.voltage as f32);
        self.cpu_usage_graph.update(msr.util as f32);
        self.cpu_avg_freq_graph.update(
            (msr.per_core_freq.iter().sum::<u64>() / msr.per_core_freq.len() as u64) as f32,
        );
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

#[derive(Default)]
pub struct Fails {
    pub msr_fail: Option<AppError>,
    pub sys_fail: Option<AppError>,
}

pub enum GpuState {
    None,
    Radeon,
    Nvidia,
}

pub struct StaticElements<'a> {
    pub cpu_title: Text<'a>,
    pub cpu_cache: Vec<(Text<'a>, Text<'a>)>,
    pub cores_threads: (Text<'a>, Text<'a>),
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

pub struct AxisState {
    pub divider: Option<u16>,
    pub split_axis: iced_aw::split::Axis,
    pub axis_state: bool,
}

impl Default for AxisState {
    fn default() -> Self {
        Self {
            divider: Default::default(),
            split_axis: Default::default(),
            axis_state: Default::default(),
        }
    }
}

impl AxisState {
    pub fn switch(&mut self) {
        if self.axis_state {
            self.axis_state = false;
            self.split_axis = iced_aw::split::Axis::Horizontal
        } else {
            self.axis_state = true;
            self.split_axis = iced_aw::split::Axis::Vertical
        }
    }

    pub fn set_divider(&mut self, x: u16) {
        self.divider = Some(x)
    }
}
