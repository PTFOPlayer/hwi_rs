use egui::Ui;

use crate::statistics::get_intel_gpu;

pub fn get_intel_ui(ui: &mut Ui) {
    match get_intel_gpu() {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    };
}