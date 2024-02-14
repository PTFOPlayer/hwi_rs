use crate::{components::chart::Graph, error::AppError};

pub struct State {
    pub fails: Fails,
    pub gpu: GpuState,
    pub graphs_switch: bool,
    pub cpu_pwr_graph: Graph,
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
            cpu_pwr_graph: Graph::new(50f32, "Cpu Power (W)"),
            cpu_temp_graph: Graph::new(100f32, "Cpu Temperature (°C)"),
            cpu_usage_graph: Graph::new(100f32, "Cpu Usage (%)"),
            cpu_avg_freq_graph: Graph::new(2500f32, "Cpu avarage frequency (MHz)"),
        }
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
