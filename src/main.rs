use eframe::{egui::CentralPanel, egui_glow, run_native, App, NativeOptions};
use egui::{self, style::Margin, Pos2, Vec2};

mod statistics;

mod overlay;
use overlay::*;

mod app_ui;
use app_ui::*;

mod settings_ui;
use settings_ui::*;

#[derive(Default)]
struct HwiRs;

impl HwiRs {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for HwiRs {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let normal = egui::Color32::from_rgba_premultiplied(15, 15, 15, 255);
        let tranparent =
            egui::Color32::from_rgba_premultiplied(15, 15, 15, get_settings().keys.opacity as u8);
        let mut my_frame = egui::containers::Frame {
            rounding: egui::Rounding {
                nw: 0.0,
                ne: 0.0,
                sw: 0.0,
                se: 0.0,
            },
            shadow: eframe::epaint::Shadow {
                extrusion: 0.0,
                color: tranparent,
            },
            fill: normal,
            stroke: egui::Stroke::new(0.0, normal),
            inner_margin: Margin {
                left: 1.,
                right: 1.,
                top: 1.,
                bottom: 1.,
            },
            outer_margin: Margin {
                left: 1.,
                right: 1.,
                top: 1.,
                bottom: 1.,
            },
        };
        if get_settings().keys.transparent {
            my_frame.stroke.color = tranparent;
            my_frame.fill = tranparent;
            my_frame.shadow.color = tranparent;
        }
        CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            if mode() {
                settings_ui(ui);
                let pos_x = get_settings().keys.overlay_x + (250 / 2) as f32;
                let pos_y = get_settings().keys.overlay_y;
                frame.set_window_pos(Pos2 { x: pos_x, y: pos_y });
                frame.set_decorations(false);
                frame.set_window_size(Vec2 { x: 250., y: 90. });
                frame.set_always_on_top(true);
                overlay_ui(ui);
            } else {
                settings_ui(ui);
                frame.set_decorations(true);
                frame.set_always_on_top(false);
                egui::ScrollArea::vertical()
                    .always_show_scroll(true)
                    .show(ui, |ui| {
                        cpu_ui(ui);
                        gpu_ui(ui);
                    });
            }
        });
    }
}
fn main() {
    statistics::get_intel_gpu();
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
        depth_buffer: 2,
        stencil_buffer: 8,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        run_and_return: false,
        event_loop_builder: None,
        shader_version: Some(egui_glow::ShaderVersion::Es300),
        centered: false,
    };
    run_native("hwi_rs", options, Box::new(|cc| Box::new(HwiRs::new(cc))));
}
