use egui::Ui;

use crate::statistics::get_radeon;

pub fn get_radeon_ui(ui: &mut Ui) -> Result<(), ()> {
    match get_radeon() {
        Ok(res) => {
            return Ok(());
        },
        Err(_) => return Err(()),
    }
}
