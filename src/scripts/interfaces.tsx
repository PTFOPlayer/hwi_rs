export interface CacheData {
  size: number,
  level: number,
  cache_type: String
}
  
export interface CpuData {
  name: String,
  logical_cores: number,
  physical_cores: number,
  power: number,
  voltage: number,
  frequency: Array<number>,
  load: number,
  temperature: number,
  cache: Array<CacheData>,
}

export interface MemData {
  total: number,
  available: number,
  used: number
}

export interface NvStats {
  spec: NvSpec,
  util: NvUtil,
  management: NvManagement
}

export interface NvSpec {
  name: String,
  memory_bus: number,
  memory: number,
  cores: number,
  arc: String,
  pci: Pci,
  cuda: CudaCapability,
  pci_e_gen: number,
  pci_e_width: number,
}

export interface Pci {
  bus: number,
  bus_id: String,
  device: number,
  domain: number,
  pci_device_id: number,
  pci_sub_system_id?: number,
}

export interface CudaCapability {
  major: number,
  minor: number,
}

export interface NvUtil {
  core_usage: number,
  memory_usage: number,
  temperature: number,
  memory_used: number,
  memory_free: number,
  current_core_clock: number,
  current_memory_clock: number,
  power_usage?: number,
}

export interface NvManagement {
  power_limit?: number,
  target_core_clock?: number,
  target_memory_clock?: number,
  default_core_clock?: number,
  default_memory_clock?: number,
  app_core_clock?: number,
  app_memory_clock?: number,
}