use crate::App;

use iced::{
    widget::{row, text, Column, Row},
    Color,
};
use rocm_smi_lib::{
    device::RocmSmiDevice, error::RocmErr, queries::performance::RsmiClkType, RocmSmi,
};

impl App {
    pub fn generate_radeon(
        &self,
    ) -> Result<iced::Element<'_, <App as iced::Application>::Message>, RocmErr> {
        let rocm = RocmSmi::init()?;
        let dev_c = rocm.get_device_count();

        let mut col = Column::new();

        for dev_id in 0..dev_c {
            let device = RocmSmiDevice::new(dev_id)?;
            let identifiers = device.get_identifiers()?;

            let title = text(format!("{}", identifiers.name))
                .size(35)
                .style(Color::from_rgb8(237, 28, 36));
            let vendor = text(format!("Vendor: {}", identifiers.vendor_name)).size(20);

            let vram_vendor =
                text(format!("Vram vendor: {}", identifiers.vram_vendor_name)).size(20);

            let vendor_info = row![vendor, vram_vendor].spacing(35);

            let usage = text(format!("Usage: {}", device.get_busy_percent()?)).size(20);
            let freq = text(format!(
                "Soc Frequency: {}",
                device.get_frequency(RsmiClkType::RsmiClkTypeSys)?.current / 1000 / 1000
            ))
            .size(20);

            let col1 = Column::new().spacing(10).push(usage);
            let col2 = Column::new().spacing(10).push(freq);
            let col3 = Column::new().spacing(10);

            let row = Row::new().spacing(35).push(col1).push(col2).push(col3);

            col = col.push(title).push(vendor_info).push(row);
        }

        Ok(col.into())
    }
}
