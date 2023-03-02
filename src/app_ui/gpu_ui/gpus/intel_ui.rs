use egui::Ui;

use crate::statistics::get_intel_gpu;

pub fn get_intel_ui(ui: &mut Ui) {
    match get_intel_gpu() {
        Ok(data) => {
            ui.label(format!("imc read: {}", data.imc_rd));
            ui.label(format!("imc write: {}", data.imc_wr));
            ui.label(format!("rcs usage: {} %", data.rcs_usg));
            ui.label(format!("bcs usage: {} %", data.bcs_usg));
            ui.label(format!("vcs usage: {} %", data.vcs_usg));
            ui.label(format!("vecs usage: {} %", data.vecs_usg));
            ui.label(format!("frequency: {} Mhz", data.freqency));
            ui.label(format!("power: {} W", data.power));
            ui.collapsing("terminology", |ui| {
                ui.label("imc -> integrated memory controler");
                ui.label("rcs -> render engine command streamer");
                ui.label("bcs -> blitter engine command streamer");
                ui.label("vcs -> vector command streamer");
                ui.label("vecs -> video enhancement engine command streamer");
            });
        },
        Err(_) => todo!(),
    };
}
