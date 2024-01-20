use crate::{components::chart::Graph, error::AppError};

pub struct State {
    pub fails: Fails,
    pub gpu: GpuState,
    pub graphs_switch: bool,
    pub cpu_pwr_graph: Graph,
    pub cpu_temp_graph: Graph,
    pub cpu_usage_graph: Graph,
}

impl Default for State {
    fn default() -> Self {
        Self {
            fails: Fails::default(),
            gpu: GpuState::None,
            graphs_switch: false,
            cpu_pwr_graph: Graph::new(50f64, "Cpu Power (W)"),
            cpu_temp_graph: Graph::new(100f64, "Cpu Temperature (°C)"),
            cpu_usage_graph: Graph::new(100f64, "Cpu Usage (%)"),
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
    Intel,
}
