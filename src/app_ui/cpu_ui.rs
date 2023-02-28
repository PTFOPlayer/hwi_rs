use std::{time::Duration};

use crate::statistics::*;
use egui::Ui;

pub fn cpu_ui(ui: &mut Ui) {
    ui.push_id(79, |ui| {
        ui.collapsing("CPU", |ui| {
            match get_cpu() {
                Ok(data) => {
                    ui.label(data.name);
                    ui.label(format!("Cores: {}", data.physical_cores));
                    ui.label(format!("Threads: {}", data.logical_cores));
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
                    ui.label(format!("load: {} %", data.load));
                    ui.label(format!("Temperature: {} °C", data.temperature));
                    ui.collapsing("advanced usage", |ui| {
                        ui.label(format!("Voltage: {}V", data.voltage));
                    });
                    ui.collapsing("advanced spec", |ui| {
                        for cache in data.cache {
                            let size = cache.associativity()
                                * cache.physical_line_partitions()
                                * cache.coherency_line_size()
                                * cache.sets();

                            ui.label(format!(
                                "{} cache level {}: {} KB",
                                cache.cache_type(),
                                cache.level(),
                                size / 1024
                            ));
                        }
                        if data.hyper_threading == 1 {
                            ui.label(format!("hyper threading: true"));
                        } else if data.hyper_threading > 1 {
                            ui.label(format!("advanced form of ht"));
                        } else {
                            ui.label(format!("hyper threading: false"));
                        }
                    });
                }
                Err(err) => {
                    ui.label(err);
                }
            };
        })
        .header_response
        .ctx
        .request_repaint_after(Duration::from_secs_f32(0.2));
    });
}
