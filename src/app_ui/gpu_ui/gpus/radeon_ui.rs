use egui::Ui;

use crate::statistics::get_radeon;

pub fn get_radeon_ui(ui: &mut Ui) -> Option<()> {
    match get_radeon() {
        Ok(res) => {
            

            return Some(());
        },
        Err(_) => return None,
    }
}
