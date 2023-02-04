use itertools::Itertools;
use std::fs;
use systemstat::{Platform, System};
pub struct CpuData {
    pub name: String,
    pub cores: String,
    pub frequency: Vec<String>,
    pub load: f32,
    pub temperature: f32,
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

            let sys = System::new();
            let load = match sys.load_average() {
                Ok(res) => res.one,
                Err(_) => todo!(),
            };

            let temperature = match sys.cpu_temp() {
                Ok(res) => res,
                Err(_) => todo!(),
            };
            return Ok(CpuData {
                name: name.to_owned(),
                cores: cores.to_owned(),
                frequency: per_core_frequency,
                load,
                temperature,
            });
        }
        Err(_) => todo!(),
    };
}
