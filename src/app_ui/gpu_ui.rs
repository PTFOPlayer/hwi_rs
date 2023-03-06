use std::time::Duration;

use egui::Ui;

mod gpus;
use self::gpus::{get_intel_ui, get_nvidia_ui, get_radeon_ui};

pub enum GpuVendor {
    Nvidia,
    Intel,
    Amd,
    None,
}

pub fn gpu_ui(ui: &mut Ui) -> GpuVendor {
    let mut gpu_check = GpuVendor::None;
    ui.collapsing("GPU", |ui| {
        if let Ok(_) = get_nvidia_ui(ui) {
            gpu_check = GpuVendor::Nvidia;
        } else if let Ok(_) = get_radeon_ui(ui) {
            gpu_check = GpuVendor::Amd;
        } else if let Ok(_) = get_intel_ui(ui) {
            gpu_check = GpuVendor::Intel;
        } else {
            gpu_check = GpuVendor::None;
        }
    })
    .header_response
    .ctx
    .request_repaint_after(Duration::from_secs_f32(0.2));
    return gpu_check;
}
