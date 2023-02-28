use std::time::Duration;

use crate::statistics::*;
use egui::Ui;

pub fn gpu_ui(ui: &mut Ui) {
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
                    ui.label(format!(
                        "cuda capability: {}.{}",
                        spec.cuda.major, spec.cuda.minor
                    ));
                    ui.label(format!("compute mode: {:?}", spec.compute));
                    ui.label(format!("cores: {}", spec.cores));
                    ui.label(format!("memory bus: {} bit", spec.memory_bus));
                    ui.label(format!("frame buffer: {} MB", spec.memory / 1024 / 1024));
                    ui.label(format!("device id: {}", spec.pci.pci_device_id));
                    ui.label(format!("bus id: {}", spec.pci.bus_id));
                    ui.label(format!("pci_e gen: {:?}", spec.pci_e_gen));
                    ui.label(format!("pci_e width: {:?}", spec.pci_e_width));
                });
            }
            Err(err) => {
                ui.label(format!("gpu error: {:?}", err));
            }
        };
    })
    .header_response
    .ctx
    .request_repaint_after(Duration::from_secs_f32(0.2));
}
