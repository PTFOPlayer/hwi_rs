use raw_cpuid::{self, CacheParametersIter};
use serde::{Deserialize, Serialize};
use std::fs;
pub struct CpuData {
    pub name: String,
    pub logical_cores: i32,
    pub physical_cores: i32,
    pub power: f32,
    pub voltage: f32,
    pub frequency: Vec<String>,
    pub load: f32,
    pub temperature: i32,
    pub cache: CacheParametersIter,
    pub hyper_threading: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CpuMsr {
    pub vendor: String,
    pub power: f32,
    pub voltage: f32,
    pub usage: f32,
    pub temperature: i32,
    pub hyper_threading: i32,
    pub logical_cores: i32,
    pub physical_cores: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Msr {
    pub cpu: CpuMsr,
}

pub fn get_cpu() -> Result<CpuData, String> {
    let mut name = "".to_owned();
    let per_core_frequency;
    match fs::read_to_string("/proc/cpuinfo") {
        Ok(res) => {
            let res_c = res.clone();
            let split = res_c.split(&['\t', '\n']);
            let splitted = split.collect::<Vec<&str>>();
            per_core_frequency = {
                let mut freq = vec![];
                let s_local = splitted.clone();
                for i in 0..s_local.len() {
                    if s_local[i].contains("MHz") {
                        freq.append(&mut vec![s_local[i + 2][2..].to_owned()]);
                    }
                    if s_local[i].contains("model name") {
                        name = (&s_local[i + 1][2..]).to_owned();
                    }
                }
                freq
            };
        }
        Err(err) => return Err(err.to_string()),
    };

            let cpuid = raw_cpuid::CpuId::new();
            let cache = match cpuid.get_cache_parameters() {
                Some(res) => res,
                None => return Err("CPU err".to_owned()),
            };

            let msr: Msr = {
                match std::fs::read_to_string("/msr_data.toml") {
                    Ok(res) => match toml::from_str(res.as_str()) {
                        Ok(res) => res,
                        Err(_) => return Err("error decoding MSR data file".to_owned()),
                    },
                    Err(_) => return Err("error reading MSR data file".to_owned()),
                }
            };

            let load = msr.cpu.usage;
            let temperature = msr.cpu.temperature;
            let logical_cores = msr.cpu.logical_cores;
            let physical_cores = msr.cpu.physical_cores;
            let voltage = msr.cpu.voltage;
            let hyper_threading = msr.cpu.hyper_threading;
            let power = msr.cpu.power;

            return Ok(CpuData {
                name: name.to_owned(),
                logical_cores,
                physical_cores,
                frequency: per_core_frequency,
                voltage,
                power,
                load,
                temperature,
                cache,
                hyper_threading,
            });

}
