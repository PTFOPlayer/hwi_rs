
use crate::tabs::{cpu::Cpu, settings::Settings, sys::Sys};

pub struct State {
    pub gpu: GpuState,
    pub cpu: Cpu,
    pub settings: Settings,
    pub sys: Sys
}

impl Default for State {
    fn default() -> Self {
        Self {
            gpu: GpuState::None,
            cpu: Cpu::new(),
            settings: Settings::new(),
            sys: Sys::new()
        }
    }
}


pub enum GpuState {
    None,
    Radeon,
    Nvidia,
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
