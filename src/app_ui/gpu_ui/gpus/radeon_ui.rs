use egui::Ui;

use crate::statistics::get_radeon;

pub fn get_radeon_ui(ui: &mut Ui) -> Option<()> {
    match get_radeon() {
        Ok(res) => {
            ui.label(format!("pci_e bus: {}", res.bus));
            ui.label(format!("usage: {}", res.usage));
            ui.label(format!("core clock: {}", res.core_clock));
            ui.label(format!("memory used (%): {}%", res.mem_percent));
            ui.label(format!("memory used: {}", res.current_mem));
            ui.label(format!("memory clock: {}", res.mem_clock));
            return Some(());
        }
        Err(_) => return None,
    }
}
