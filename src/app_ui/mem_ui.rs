use std::time::Duration;
use egui::Ui;
use crate::statistics::get_mem;


pub fn mem_ui(ui: &mut Ui) {
    ui.collapsing("Memory", |ui| {
        match get_mem() {
            Ok(data) => {
                ui.label(format!("total: {} MB", data.total / 1024));
                ui.label(format!("available: {} MB", data.available / 1024));
                ui.label(format!("used: {} MB", data.used / 1024));
            },
            Err(err) => {
                ui.label(err);
            },
        };
    })
    .header_response
    .ctx
    .request_repaint_after(Duration::from_secs_f32(0.2));
}
