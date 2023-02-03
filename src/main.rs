use std::thread;
use std::time::Duration;
use eframe::{egui::CentralPanel, run_native, App, NativeOptions};
use egui;

mod statistics;
use statistics::*;

#[derive(Default)]
struct HwiRs;

impl HwiRs {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for HwiRs {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.push_id(79, |ui| {
                ui.collapsing("CPU", |ui| {
                match get_cpu() {
                    Ok(data) => {
                        ui.label(data.name);
                        ui.label(format!("{} Mhz", data.frequency));
                        ui.label(format!("{} %", data.load));
                        ui.label(format!("{} C", data.temperature));
                    }
                    Err(_) => {
                        ui.label("cpu error");
                    }
                };
            });
            });
            ui.collapsing("GPU", |ui| {
                match get_nv() {
                    Ok(data) => {
                        ui.label(format!("{}", data.name));
                        ui.label(format!("Gpu core usage: {}", data.usage.gpu));
                        ui.label(format!("Gpu memory usage: {}", data.usage.memory));
                        ui.label(format!("Gpu temperature: {}", data.temperature));
                    }
                    Err(_) => {
                        ui.label("gpu error");
                    }
                };
            });
            thread::sleep(Duration::from_secs_f32(0.05));
            ctx.request_repaint();
        });
    }
}

fn main() {
    let options = NativeOptions::default();
    run_native("hwi_rs", options, Box::new(|cc| Box::new(HwiRs::new(cc))));
}
