use nvml_wrapper::{
    enum_wrappers::device, error::NvmlError, Nvml,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct NvStats {
    pub spec: NvSpec,
    pub util: NvUtil,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NvSpec {
    pub name: String,
    pub memory_bus: u32,
    pub memory: u64,
    pub cores: u32,
    pub arc: String,
    pub pci: Pci,
    pub cuda: CudaCapability,
    pub pci_e_gen: u32,
    pub pci_e_width: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CudaCapability {
    pub major: i32,
    pub minor: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NvUtil {
    pub core_usage: u32,
    pub memory_usage: u32,
    pub temperature: u32,
    pub memory_used: u64,
    pub memory_free: u64,
    pub current_core_clock: u32,
    pub current_memory_clock: u32,
}


#[derive(Deserialize, Serialize, Clone)]
pub struct Pci {
    pub bus: u32,
    pub bus_id: String,
    pub device: u32,
    pub domain: u32,
    pub pci_device_id: u32,
    pub pci_sub_system_id: Option<u32>,
}

pub fn get_nv() -> Result<NvStats, NvmlError> {
    let nvml = Nvml::init()?;

    let device = nvml.device_by_index(0)?;

    let name = device.name()?;

    let memory_bus = device.memory_bus_width()?;
    let cores = device.num_cores()?;
    let arc = device.architecture()?.to_string();
    let pci_info = device.pci_info()?;
    let pci = Pci {
        bus: pci_info.bus,
        bus_id: pci_info.bus_id,
        device: pci_info.device,
        domain: pci_info.domain,
        pci_device_id: pci_info.pci_device_id,
        pci_sub_system_id: pci_info.pci_sub_system_id,
    };

    let cuda_major = device.cuda_compute_capability()?.major;
    let cuda_minor = device.cuda_compute_capability()?.minor;
    let cuda = CudaCapability {
        major: cuda_major,
        minor: cuda_minor,
    };

    let pci_e_gen = device.current_pcie_link_gen()?;
    let pci_e_width = device.current_pcie_link_width()?;

    let current_core_clock = device.clock(device::Clock::Graphics, device::ClockId::Current)?;
    let current_memory_clock = device.clock(device::Clock::Memory, device::ClockId::Current)?;
    let memory = device.memory_info()?;
    let core_usage = device.utilization_rates()?.gpu;
    let memory_usage = device.utilization_rates()?.memory;
    let temperature =
        device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)?;

    let data = NvStats {
        spec: NvSpec {
            name,
            memory_bus,
            memory: memory.total,
            cores,
            arc,
            pci,
            cuda,
            pci_e_gen,
            pci_e_width,
        },
        util: NvUtil {
            core_usage,
            memory_usage,
            temperature,
            memory_used: memory.used,
            memory_free: memory.free,
            current_core_clock,
            current_memory_clock,
        },
    };
    Ok(data)
}
