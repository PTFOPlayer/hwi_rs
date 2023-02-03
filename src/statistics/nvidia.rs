use nvml_wrapper::{error::NvmlError, struct_wrappers::device::Utilization, Nvml};
pub struct NvData {
    pub name: String,
    pub usage: Utilization,
    pub temperature: u32,
}
pub fn get_nv() -> Result<NvData, NvmlError> {
    let nvml = Nvml::init()?;
    // Get the first `Device` (GPU) in the system
    let device = nvml.device_by_index(0)?;

    let name = device.name()?;
    let usage = device.utilization_rates()?;
    let temperature =
        device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)?;
    let data = NvData {
        name,
        usage,
        temperature,
    };
    Ok(data)
}
