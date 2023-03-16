use std::time::Duration;

use egui::Ui;

mod gpus;
use self::gpus::{get_intel_ui, get_nvidia_ui, get_radeon_ui};

pub fn gpu_ui(ui: &mut Ui) {
    ui.collapsing("GPU", |ui| {
        // allows handling of ui results
        // currently not used
        match get_nvidia_ui(ui) {
            Some(_) => {
                return;
            }
            None => {}
        };
        match get_radeon_ui(ui) {
            Some(_) => {
                return;
            }
            None => {}
        }
        match get_intel_ui(ui) {
            Some(_) => {
                return;
            }
            None => {}
        }
    })
    .header_response
    .ctx
    .request_repaint_after(Duration::from_secs_f32(0.2));
}
