use crate::{components::chart::Graph, error::AppError};

pub struct State {
    pub fails: Fails,
    pub gpu: GpuState,
    pub cpu_pwr_graph: Graph<f64>,
    pub cpu_temp_graph: Graph<f32>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            gpu: GpuState::None,
            cpu_pwr_graph: Graph::new(100f64),
            cpu_temp_graph: Graph::new(100f32),
            fails: Fails::default(),    
        }
    }
}

#[derive(Default)]
pub struct Fails {
    pub msr_fail : Option<AppError>,
    pub sys_fail: Option<AppError>
}

pub enum GpuState {
    None,
    Radeon,
    Nvidia,
    Intel,
}
