use raw_cpuid;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Clone)]
pub struct CacheData {
    pub size: i64,
    pub level: u8,
    pub cache_type: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CpuData {
    pub name: String,
    pub logical_cores: i32,
    pub physical_cores: i32,
    pub power: f32,
    pub voltage: f32,
    pub frequency: Vec<i64>,
    pub load: f32,
    pub temperature: i32,
    pub cache: Vec<CacheData>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MemMsr {
    pub mem_total: i32,
    pub mem_free: i32,
    pub mem_used: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MemData {
    pub total: i32,
    pub available: i32,
    pub used: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CpuMsr {
    pub vendor: String,
    pub name: String,
    pub freq: i64,
    pub util: f32,
    pub threads: i32,
    pub cores: i32,
    pub temperature: f32,
    pub voltage: f32,
    pub package_power: f32,
    pub per_core_freq: Vec<i64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Msr {
    pub cpu: CpuMsr,
    pub memory: MemMsr,
}

pub fn get_cpu() -> Result<CpuData, String> {
    let req = reqwest::blocking::get("http://localhost:8000");

    let cpuid = raw_cpuid::CpuId::new();
    let cache = match cpuid.get_cache_parameters() {
        Some(res) => res,
        None => return Err("CPU err".to_owned()),
    };

    let mut cache_vec = vec![];
    for c in cache {
        let size =
            c.associativity() * c.physical_line_partitions() * c.coherency_line_size() * c.sets();
        let size = size as i64;
        let level = c.level();
        let cache_type = c.cache_type().to_string();

        cache_vec.push(CacheData {
            size,
            level,
            cache_type,
        });
    }

    let msr: Msr = {
        let res = match req {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        };

        let data = match res?.text() {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        };
        let msr = match serde_json::from_str(&data?) {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        };
        msr?
    };
    
    let name = msr.cpu.name;
    let load = msr.cpu.util;
    let temperature = msr.cpu.temperature;
    let logical_cores = msr.cpu.threads;
    let physical_cores = msr.cpu.cores;
    let voltage = msr.cpu.voltage;
    let per_core_frequency = msr.cpu.per_core_freq;
    let power = msr.cpu.package_power;
    
    return Ok(CpuData {
        name,
        logical_cores,
        physical_cores,
        frequency: per_core_frequency,
        voltage,
        power,
        load,
        temperature: temperature as i32,
        cache: cache_vec,
    });
}

pub fn get_mem() -> Result<MemData, String> {
    let req = reqwest::blocking::get("http://localhost:8000");

    let msr: Msr = {
        let res = match req {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        };

        let data = match res?.text() {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        };
        let msr = match serde_json::from_str(&data?) {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        };
        msr?
    };

    return Ok(MemData {
        total: msr.memory.mem_total,
        available: msr.memory.mem_free,
        used: msr.memory.mem_used,
    });
}
