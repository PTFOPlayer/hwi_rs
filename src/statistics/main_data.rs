pub async fn get_data(url: String) -> Result<MsrData, AppError> {
    let req = reqwest::get(url).await;
    let msr: MsrData = {
        let res = req?.text().await?;
        let msr: Data = serde_json::from_str(&res)?;
        msr.core
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
    pub freq: u64,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Data {
    core: MsrData,
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
