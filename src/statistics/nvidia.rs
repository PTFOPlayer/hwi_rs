use nvml_wrapper::{
    enum_wrappers::device,
    enums::device::DeviceArchitecture,
    error::NvmlError,
    struct_wrappers::device::{PciInfo, Utilization},
    Nvml,
};
pub struct NvData {
    pub spec: NvSpec,
    pub util: NvUtil,
}
pub struct NvSpec {
    pub name: String,
    pub memory_bus: u32,
    pub memory: u64,
    pub cores: u32,
    pub arc: DeviceArchitecture,
    pub pci: PciInfo,
}

pub struct NvUtil {
    pub core_usage: Utilization,
    pub temperature: u32,
    pub memory_used: u64,
    pub memory_free: u64,
    pub current_core_clock: u32,
    pub current_memory_clock: u32,
}

pub fn get_nv() -> Result<NvData, NvmlError> {
    let nvml = Nvml::init()?;
    // Get the first `Device` (GPU) in the system
    let device = nvml.device_by_index(0)?;

    let name = device.name()?;

    let memory_bus = device.memory_bus_width()?;
    let cores = device.num_cores()?;
    let arc = device.architecture()?;
    let pci = device.pci_info()?;

    let current_core_clock = device.clock(device::Clock::Graphics, device::ClockId::Current)?;
    let current_memory_clock = device.clock(device::Clock::Memory, device::ClockId::Current)?;
    let memory = device.memory_info()?;
    let core_usage = device.utilization_rates()?;
    let temperature =
        device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)?;

    let data = NvData {
        spec: NvSpec {
            name,
            memory_bus,
            memory: memory.total,
            cores,
            arc,
            pci,
        },
        util: NvUtil {
            core_usage,
            temperature,
            memory_used: memory.used,
            memory_free: memory.free,
            current_core_clock,
            current_memory_clock,
        },
    };
    Ok(data)
}
