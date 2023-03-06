use egui::Ui;
use serde::{Deserialize, Serialize};
use std::{ops::RangeInclusive, process::exit};
use toml;

#[derive(Deserialize, Serialize, Clone)]
pub struct Keys {
    pub mode: bool,
    pub transparent: bool,
    pub settings_always: bool,
    pub opacity: u8,
    pub overlay_x: f32,
    pub overlay_y: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub keys: Keys,
}

static PATH: &str = "~/.config/hwi_rs/settings.toml";

pub fn get_settings() -> Config {
    let file = std::fs::read_to_string(PATH);
    match file {
        Ok(res) => toml::from_str(res.as_str()).unwrap(),
        Err(_) => {
            println!("error reading settings");
            exit(-1)
        }
    }
}

pub fn settings_ui(ui: &mut Ui) {
    ui.horizontal(|ui| {
        let current_settings = get_settings();
        let overlay = &mut current_settings.keys.mode.clone();
        if ui.checkbox(overlay, "overlay").changed() {
            let mut confing_to_write = current_settings.clone();
            confing_to_write.keys.mode = !current_settings.keys.mode;
            _ = std::fs::write(
                PATH,
                toml::to_string(&confing_to_write).unwrap(),
            );
        };
        if !current_settings.keys.mode || current_settings.keys.settings_always {
            ui.menu_button("Settings", |ui| {
                let t_state = &mut current_settings.keys.transparent.clone();
                if ui.checkbox(t_state, "transparent").changed() {
                    let mut confing_to_write = current_settings.clone();
                    confing_to_write.keys.transparent = !current_settings.keys.transparent;
                    _ = std::fs::write(
                        PATH,
                        toml::to_string(&confing_to_write).unwrap(),
                    );
                }
                let s_state = &mut current_settings.keys.settings_always.clone();
                if ui.checkbox(s_state, "settings always on").changed() {
                    let mut confing_to_write = current_settings.clone();
                    confing_to_write.keys.settings_always = !current_settings.keys.settings_always;
                    _ = std::fs::write(
                        PATH,
                        toml::to_string(&confing_to_write).unwrap(),
                    );
                }
                let value = &mut current_settings.keys.opacity.clone();
                if ui
                    .add(egui::widgets::Slider::new(
                        value,
                        RangeInclusive::new(0, 255),
                    ))
                    .changed()
                {
                    let mut confing_to_write = current_settings.clone();
                    confing_to_write.keys.opacity = *value;
                    _ = std::fs::write(
                        PATH,
                        toml::to_string(&confing_to_write).unwrap(),
                    );
                }
                ui.horizontal(|ui| {
                    ui.label("overlay x");
                    let o_x = &mut format!("{}", current_settings.keys.overlay_x);
                    if ui.text_edit_singleline(o_x).changed() {
                        let mut confing_to_write = current_settings.clone();
                        match o_x.parse::<f32>() {
                            Ok(res) => {
                                confing_to_write.keys.overlay_x = res;
                                _ = std::fs::write(
                                    PATH,
                                    toml::to_string(&confing_to_write).unwrap(),
                                );
                            }
                            Err(_) => {
                                confing_to_write.keys.overlay_x = 0.0;
                                _ = std::fs::write(
                                    PATH,
                                    toml::to_string(&confing_to_write).unwrap(),
                                );
                            }
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("overlay y");
                    let o_y = &mut format!("{}", current_settings.keys.overlay_y);
                    if ui.text_edit_singleline(o_y).changed() {
                        let mut confing_to_write = current_settings.clone();
                        match o_y.parse::<f32>() {
                            Ok(res) => {
                                confing_to_write.keys.overlay_y = res;
                                _ = std::fs::write(
                                    PATH,
                                    toml::to_string(&confing_to_write).unwrap(),
                                );
                            }
                            Err(_) => {
                                confing_to_write.keys.overlay_y = 0.0;
                                _ = std::fs::write(
                                    PATH,
                                    toml::to_string(&confing_to_write).unwrap(),
                                );
                            }
                        }
                    }
                });
            });
        }
    });
}

pub fn mode() -> bool {
    get_settings().keys.mode.clone()
}
