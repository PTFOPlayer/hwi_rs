use std::time::Duration;

use egui::Ui;

mod gpus;
use self::gpus::get_nvidia_ui;


pub fn gpu_ui(ui: &mut Ui) {
    ui.collapsing("GPU", |ui| {
        get_nvidia_ui(ui);
    })
    .header_response
    .ctx
    .request_repaint_after(Duration::from_secs_f32(0.2));
}
