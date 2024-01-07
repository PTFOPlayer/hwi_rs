pub struct State {
    pub gpu: GpuState
}

impl Default for State {
    fn default() -> Self {
        Self { gpu: GpuState::None }
    }
}


pub enum GpuState {
    None, 
    Radeon,
    Nvidia,
    Intel
}

