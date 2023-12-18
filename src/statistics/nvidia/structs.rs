use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct NvStats {
    pub spec: NvSpec,
    pub util: NvUtil,
    pub management: NvManagement,
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
pub struct Pci {
    pub bus: u32,
    pub bus_id: String,
    pub device: u32,
    pub domain: u32,
    pub pci_device_id: u32,
    pub pci_sub_system_id: Option<u32>,
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
    pub power_usage: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NvManagement {
    pub power_limit: Option<u32>,
    pub target_core_clock: Option<u32>,
    pub target_memory_clock: Option<u32>,
    pub default_core_clock: Option<u32>,
    pub default_memory_clock: Option<u32>,
    pub app_core_clock: Option<u32>,
    pub app_memory_clock: Option<u32>,
}