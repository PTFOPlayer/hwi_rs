use eframe::{egui::CentralPanel, run_native, App, NativeOptions};
use egui::{self, Pos2, Ui, Vec2};
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

static mut UI_T: bool = false;
impl App for HwiRs {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        
        CentralPanel::default().show(ctx, |ui| unsafe {
            ui.horizontal(|ui| {
                ui.checkbox(&mut UI_T, "overlay")
            });
            if UI_T {
                frame.set_window_pos(Pos2 { x: 100., y: 0. });
                frame.set_decorations(false);
                frame.set_window_size(Vec2 { x: 320., y: 90. });
                frame.set_always_on_top(true);
                overlay_ui(ui);
            } else {
                frame.set_decorations(true);
                frame.set_always_on_top(false);
                frame.set_window_size(Vec2 { x: 450., y: 250. });
                cpu_ui(ui);
                gpu_ui(ui);
            }
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

fn overlay_ui(ui: &mut Ui) {
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
                ui.label(format!("{}°C", data.temperature));
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
                    "{}MHz\t{}%\t{}°C",
                    data.util.current_core_clock, data.util.core_usage.gpu, data.util.temperature
                ));
            })
            .response
            .ctx
            .request_repaint_after(Duration::from_secs_f32(0.5));
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(118, 185, 000), "GPU mem");
                ui.label(format!(
                    "{}MHz\t{}%\t{}MB",
                    data.util.current_memory_clock, data.util.core_usage.memory, data.util.memory_used / 1024 / 1024
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

fn gpu_ui(ui: &mut Ui) {
    ui.collapsing("GPU", |ui| {
        match get_nv() {
            Ok(data) => {
                let (spec, util) = (data.spec, data.util);
                ui.label(format!("{}", spec.name));
                ui.label(format!("core usage: {} %", util.core_usage.gpu));
                ui.label(format!("core clock: {} MHz", util.current_core_clock));
                let mut col = false;
                ui.horizontal(|ui| {
                    ui.collapsing("", |ui| {
                        col = true;
                        ui.label(format!("memory usage: {} %", util.core_usage.memory));
                        ui.label(format!(
                            "memory free: {} Mb",
                            util.memory_free / 1024 / 1024
                        ));
                        ui.label(format!(
                            "memory used: {} Mb",
                            util.memory_used / 1024 / 1024
                        ));
                    });
                    if !col {
                        ui.label(format!("memory usage: {} %", util.core_usage.memory));
                    }
                });
                ui.label(format!("memory clock: {} MHz", util.current_memory_clock));
                ui.label(format!("temperature: {} °C", util.temperature));
                ui.collapsing("advanced usage", |ui| {
                    ui.label(format!(""));
                });
                ui.collapsing("spec", |ui| {
                    ui.label(format!("architecture: {}", spec.arc));
                    ui.label(format!("cores: {}", spec.cores));
                    ui.label(format!("memory bus: {} bit", spec.memory_bus));
                    ui.label(format!("bus id: {}", spec.pci.bus_id));
                    ui.label(format!("device id: {}", spec.pci.pci_device_id));
                });
            }
            Err(err) => {
                ui.label(format!("gpu error: {:?}", err));
            }
        };
    })
    .header_response
    .ctx
    .request_repaint_after(Duration::from_secs_f32(0.1));
}

fn main() {
    let options = NativeOptions {
        always_on_top: true,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Some(Vec2 { x: 450., y: 250. }),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        mouse_passthrough: false,
        vsync: false,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        run_and_return: false,
        event_loop_builder: None,
        shader_version: None,
        centered: false,
    };
    run_native("hwi_rs", options, Box::new(|cc| Box::new(HwiRs::new(cc))));
}
