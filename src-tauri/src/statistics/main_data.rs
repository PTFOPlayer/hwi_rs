use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheData {
    pub size: i64,
    pub level: u8,
    pub cache_type: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MemMsr {
    pub mem_total: i32,
    pub mem_free: i32,
    pub mem_used: i32,
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
    pub cache: Vec<CacheData>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Msr {
    pub cpu: CpuMsr,
    pub memory: MemMsr,
}

pub fn get_msr() -> Result<Msr, String> {
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
    
    Ok(msr)
}