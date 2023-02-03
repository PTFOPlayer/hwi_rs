use raw_cpuid;
use systemstat::{System, Platform};
use cpuid;
pub struct CpuData {
    pub name: String,
    pub frequency: i32,
    pub load: f32,
    pub temperature: f32,
}

pub fn get_cpu() -> Result<CpuData, String> {
    let rcpu = raw_cpuid::CpuId::new();
    let name = match rcpu.get_processor_brand_string() {
        Some(res) => res.as_str().to_owned(),
        None => return Err("name error".to_owned()),
    };
    
    let sys = System::new();
    let load = match sys.load_average() {
        Ok(res) => res.one ,
        Err(_) => todo!(),
    };
    
    let frequency = match cpuid::clock_frequency() {
        Some(res) => res,
        None => todo!(),
    };

    let temperature = match sys.cpu_temp() {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    
    return Ok(CpuData {
        name,
        frequency,
        load,
        temperature,
    });
}
