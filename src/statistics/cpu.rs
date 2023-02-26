use itertools::Itertools;
use raw_cpuid::{self, CacheParametersIter};
use serde::{Deserialize, Serialize};
use std::fs;
pub struct CpuData {
    pub name: String,
    pub cores: String,
    pub frequency: Vec<String>,
    pub load: f32,
    pub temperature: i32,
    pub cache: CacheParametersIter,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CpuMsr {
    pub power: f32,
    pub voltage: f32,
    pub usage: f32,
    pub temperature: i32,
    pub thread_count: i32,
    pub hyper_threading: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Msr {
    pub cpu: CpuMsr,
}

pub fn get_cpu() -> Result<CpuData, String> {
    match fs::read_to_string("/proc/cpuinfo") {
        Ok(res) => {
            let res_c = res.clone();
            let splitted = res_c.split(&['\t', '\n']).collect::<Vec<&str>>();
            let mut name = "";
            let mut cores = "";
            let per_core_frequency = {
                let mut freq = vec![];
                let s_local = splitted.clone();
                for i in 0..s_local.len() {
                    if s_local[i].contains("MHz") {
                        freq.append(&mut vec![s_local[i + 2][2..].to_owned()]);
                    }
                }
                freq
            };
            let splitted = splitted.into_iter().unique().collect::<Vec<&str>>();
            for i in 0..splitted.len() {
                if splitted[i].contains("model name") {
                    name = &splitted[i + 1][2..];
                }
                if splitted[i].contains("cpu cores") {
                    cores = &splitted[i + 1][2..];
                }
            }

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

            return Ok(CpuData {
                name: name.to_owned(),
                cores: cores.to_owned(),
                frequency: per_core_frequency,
                load,
                temperature,
                cache,
            });
        }
        Err(err) => return Err(err.to_string()),
    };
}
