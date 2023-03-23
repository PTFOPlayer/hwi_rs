pub struct RadeonStats {
    pub usage: String,
    pub current_mem: String,
    pub mem_percent: String,
    pub bus: String,
    pub mem_clock: String,
    pub core_clock: String,
}

pub fn get_radeon() -> Result<RadeonStats, ()> {
    let file = std::fs::read_to_string("./radeon");
    match file {
        Ok(res) => {
            if res.contains("err") {
                return Err(());
            }

            let lines = res.lines().collect::<Vec<&str>>();
            if lines.len() < 2 {
                return Err(());
            }

            let data_line = lines[1];
            let segments = data_line.split([' ', ',']).collect::<Vec<&str>>();

            let bus = segments[2].to_owned();
            let usage = segments[5].to_owned();

            let current_mem = segments[28].to_owned();
            let mem_percent = segments[26].to_owned();

            let mem_clock = segments[47].to_owned();
            let core_clock = segments[51].to_owned();

            return Ok(RadeonStats {
                usage,
                current_mem,
                mem_percent,
                bus,
                mem_clock,
                core_clock,
            });
        }
        Err(_) => return Err(()),
    }
}
