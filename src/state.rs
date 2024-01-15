use crate::{components::chart::Graph, error::AppError};

pub struct State {
    pub fails: Fails,
    pub gpu: GpuState,
    pub cpu_pwr_graph: Graph,
    pub cpu_temp_graph: Graph,
}

impl Default for State {
    fn default() -> Self {
        Self {
            gpu: GpuState::None,
            cpu_pwr_graph: Graph::new(100f64, "Cpu Power"),
            cpu_temp_graph: Graph::new(100f64, "Cpu Temperature"),
            fails: Fails::default(),
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
