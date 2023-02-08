use egui::Ui;
use std::time::Duration;

use crate::statistics::*;

pub fn overlay_ui(ui: &mut Ui) {
    match get_cpu() {
        Ok(data) => {
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(000, 104, 181), "CPU");
                ui.label(format!("{}MHz", data.frequency[0]));
                let load = (data.load * 100.).round() as i32;
                let load_prev = 0;
                if load != 0 {
                    ui.label(format!("{}%", load));
                } else {
                    ui.label(format!("{}%", load_prev));
                }
                if data.temperature < 35. {
                    ui.label(format!("{}°C", data.temperature));
                } else {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 50, 50),
                        format!("{}°C", data.temperature),
                    );
                }
            })
            .response
            .ctx
            .request_repaint_after(Duration::from_secs_f32(0.5));
        }
        Err(_) => {
            ui.label("cpu error");
        }
    }
    match get_nv() {
        Ok(data) => {
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(118, 185, 000), "GPU core");
                ui.label(format!(
                    "{}MHz\t{}%",
                    data.util.current_core_clock, data.util.core_usage.gpu
                ));
                if data.util.temperature < 35 {
                    ui.label(format!("{}°C", data.util.temperature));
                } else {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 50, 50),
                        format!("{}°C", data.util.temperature),
                    );
                }
            })
            .response
            .ctx
            .request_repaint_after(Duration::from_secs_f32(0.5));
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(118, 185, 000), "GPU mem");
                ui.label(format!(
                    "{}MHz\t{}%\t{}MB",
                    data.util.current_memory_clock,
                    data.util.core_usage.memory,
                    data.util.memory_used / 1024 / 1024
                ));
            })
            .response
            .ctx
            .request_repaint_after(Duration::from_secs_f32(0.5));
        }
        Err(_) => {
            ui.label("gpu error");
        }
    }
}
