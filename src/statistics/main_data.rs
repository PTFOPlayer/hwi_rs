pub async fn get_data(url: String) -> Result<MsrData, AppError> {
    let req = reqwest::get(url).await;
    let msr = {
        let res = req?.text().await?;
        serde_json::from_str(&res)?
    };
    Ok(msr)
}

use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MsrData {
    pub voltage: f64,
    pub package_power: f64,
    pub vendor: String,
    pub name: String,
    pub util: f64,
    pub threads: i32,
    pub cores: i32,
    pub temperature: f32,
    pub per_core_freq: Vec<u64>,
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_used: u64,
    pub cache: Vec<CacheData>,
}

impl Default for MsrData {
    fn default() -> Self {
        Self {
            voltage: Default::default(),
            package_power: Default::default(),
            vendor: Default::default(),
            name: Default::default(),
            util: Default::default(),
            threads: Default::default(),
            cores: Default::default(),
            temperature: Default::default(),
            per_core_freq: vec![0],
            mem_total: Default::default(),
            mem_free: Default::default(),
            mem_used: Default::default(),
            cache: Default::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheData {
    pub size: i64,
    pub level: u8,
    pub cache_type: String,
}

pub async fn get_system_data(url: String) -> Result<SystemInfo, AppError> {
    let req = reqwest::get(url + "/system").await;
    let msr = {
        let res = req?.text().await?;
        serde_json::from_str(&res)?
    };
    Ok(msr)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SystemInfo {
    pub host_name: String,
    pub boot_time: u64,
    pub distro_id: String,
    pub kernel_version: String,
    pub os_version: String,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            host_name: "DefaultHostName".to_owned(),
            boot_time: Default::default(),
            distro_id: Default::default(),
            kernel_version: Default::default(),
            os_version: Default::default(),
        }
    }
}
