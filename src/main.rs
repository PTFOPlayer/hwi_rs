use eframe::{egui::CentralPanel, run_native, App, NativeOptions};
use egui::{self, Ui};
use std::thread;
use std::time::Duration;

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
            cpu_ui(ui);
            gpu_ui(ui);
            thread::sleep(Duration::from_secs_f32(0.05));
            ctx.request_repaint();
        });
    }
}

fn cpu_ui(ui: &mut Ui) {
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
                            ui.label(format!("Frequency: {} Mhz", data.frequency[0]));
                        }
                    });
                    ui.label(format!("Avg one minut load: {} %", data.load));
                    ui.label(format!("Temperature: {} C", data.temperature));
                }
                Err(_) => {
                    ui.label("cpu error");
                }
            };
        });
    });
}

fn gpu_ui(ui: &mut Ui) {
    ui.collapsing("GPU", |ui| {
        match get_nv() {
            Ok(data) => {
                let (spec, util) = (data.spec, data.util);
                ui.label(format!("{}", spec.name));
                ui.label(format!("core usage: {}", util.core_usage.gpu));
                ui.label(format!("core clock: {}", util.current_core_clock));
                let mut col = false;
                ui.horizontal(|ui| {
                    ui.collapsing("", |ui| {
                        col = true;
                        ui.label(format!("memory usage: {}", util.core_usage.memory));
                        ui.label(format!("memory free: {}", util.memory_free));
                        ui.label(format!("memory used: {}", util.memory_used));
                    });
                    if !col {
                        ui.label(format!("memory usage: {}", util.core_usage.memory));
                    }
                });
                ui.label(format!("memory clock: {}", util.current_memory_clock));
                ui.label(format!("temperature: {}", util.temperature));
                ui.collapsing("advanced usage", |ui| {
                    ui.label(format!(""));
                });
                ui.collapsing("spec", |ui| {
                    ui.label(format!("architecture: {}", spec.arc));
                    ui.label(format!("cores: {}", spec.cores));
                    ui.label(format!("memory bus: {}", spec.memory_bus));
                });
            }
            Err(_) => {
                ui.label("gpu error");
            }
        };
    });
}

fn main() {
    let options = NativeOptions::default();
    run_native("hwi_rs", options, Box::new(|cc| Box::new(HwiRs::new(cc))));
}
