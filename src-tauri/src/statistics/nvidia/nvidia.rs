use crate::statistics::nvidia::structs::*;
use lazy_static::lazy_static;
use nvml_wrapper::{enum_wrappers::device, error::NvmlError, Nvml};

lazy_static! {
    static ref NVML_R: Result<Nvml, NvmlError> = Nvml::init();
}

pub fn get_nv() -> Result<NvStats, NvmlError> {
    match NVML_R.as_ref() {
        Ok(nvml) => {
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

            let current_core_clock =
                device.clock(device::Clock::Graphics, device::ClockId::Current)?;
            let current_memory_clock =
                device.clock(device::Clock::Memory, device::ClockId::Current)?;
            let memory = device.memory_info()?;
            let core_usage = device.utilization_rates()?.gpu;
            let memory_usage = device.utilization_rates()?.memory;
            let temperature =
                device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)?;

            let power_limit = match device.enforced_power_limit() {
                Ok(pwr) => Some(pwr),
                Err(_) => None,
            };

            let power_usage = match device.power_usage() {
                Ok(pwr) => Some(pwr),
                Err(_) => None,
            };

            let target_core_clock =
                match device.clock(device::Clock::Graphics, device::ClockId::TargetAppClock) {
                    Ok(res) => Some(res),
                    Err(_) => None,
                };
            let target_memory_clock =
                match device.clock(device::Clock::Memory, device::ClockId::TargetAppClock) {
                    Ok(res) => Some(res),
                    Err(_) => None,
                };

            let default_core_clock =
                match device.clock(device::Clock::Graphics, device::ClockId::DefaultAppClock) {
                    Ok(res) => Some(res),
                    Err(_) => None,
                };
            let default_memory_clock =
                match device.clock(device::Clock::Memory, device::ClockId::DefaultAppClock) {
                    Ok(res) => Some(res),
                    Err(_) => None,
                };

            let app_core_clock = match device.applications_clock(device::Clock::Graphics) {
                Ok(res) => Some(res),
                Err(_) => None,
            };

            let app_memory_clock = match device.applications_clock(device::Clock::Memory) {
                Ok(res) => Some(res),
                Err(_) => None,
            };

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
                    power_usage,
                },
                management: NvManagement {
                    power_limit,
                    target_core_clock,
                    target_memory_clock,
                    default_core_clock,
                    default_memory_clock,
                    app_core_clock,
                    app_memory_clock,
                },
            };
            Ok(data)
        }
        Err(err) => match err {
            NvmlError::SetReleaseFailed => Err(NvmlError::SetReleaseFailed),
            NvmlError::GetPciInfoFailed => Err(NvmlError::GetPciInfoFailed),
            NvmlError::PciInfoToCFailed => Err(NvmlError::PciInfoToCFailed),
            NvmlError::Uninitialized => Err(NvmlError::Uninitialized),
            NvmlError::InvalidArg => Err(NvmlError::InvalidArg),
            NvmlError::NotSupported => Err(NvmlError::NotSupported),
            NvmlError::NoPermission => Err(NvmlError::NoPermission),
            NvmlError::NotFound => Err(NvmlError::NotFound),
            NvmlError::InsufficientPower => Err(NvmlError::InsufficientPower),
            NvmlError::DriverNotLoaded => Err(NvmlError::DriverNotLoaded),
            NvmlError::Timeout => Err(NvmlError::Timeout),
            NvmlError::IrqIssue => Err(NvmlError::IrqIssue),
            NvmlError::LibraryNotFound => Err(NvmlError::LibraryNotFound),
            NvmlError::FunctionNotFound => Err(NvmlError::FunctionNotFound),
            NvmlError::CorruptedInfoROM => Err(NvmlError::CorruptedInfoROM),
            NvmlError::GpuLost => Err(NvmlError::GpuLost),
            NvmlError::ResetRequired => Err(NvmlError::ResetRequired),
            NvmlError::OperatingSystem => Err(NvmlError::OperatingSystem),
            NvmlError::LibRmVersionMismatch => Err(NvmlError::LibRmVersionMismatch),
            NvmlError::InUse => Err(NvmlError::InUse),
            NvmlError::InsufficientMemory => Err(NvmlError::InsufficientMemory),
            NvmlError::NoData => Err(NvmlError::NoData),
            NvmlError::VgpuEccNotSupported => Err(NvmlError::VgpuEccNotSupported),
            _ => Err(NvmlError::Unknown),
        },
    }
}
