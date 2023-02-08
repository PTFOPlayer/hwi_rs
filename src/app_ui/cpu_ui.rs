use crate::statistics::*;
use egui::Ui;
use systemstat::Duration;

pub fn cpu_ui(ui: &mut Ui) {
    ui.push_id(79, |ui| {
        ui.collapsing("CPU", |ui| {
            match get_cpu() {
                Ok(data) => {
                    ui.label(data.name);
                    ui.label(format!("Cores: {}", data.cores));
                    let mut col = false;
                    ui.horizontal(|ui| {
                        ui.collapsing("", |ui| {
                            col = true;
                            for i in 0..data.frequency.len() {
                                ui.label(format!("{}: {}", i, data.frequency[i]));
                            }
                        });
                        if !col {
                            ui.label(format!("Frequency: {} MHz", data.frequency[0]));
                        }
                    });
                    ui.label(format!("Avg one minut load: {} %", data.load));
                    ui.label(format!("Temperature: {} °C", data.temperature));
                }
                Err(_) => {
                    ui.label("cpu error");
                }
            };
        })
        .header_response
        .ctx
        .request_repaint_after(Duration::from_secs_f32(0.1));
    });
}
