use std::process::Command;

pub struct IgData {
    pub freqency: String,
    pub power: String,
    pub imc_rd: String,
    pub imc_wr: String,
    pub rcs_usg: String,
    pub bcs_usg: String,
    pub vcs_usg: String,
    pub vecs_usg: String,
}

// sudo intel_gpu_top -o - | awk '{print>"current";close("current")}'
pub fn get_intel_gpu() -> Result<IgData, String> {
    let mut getter = Command::new("cat");
    getter.arg("./current");
    let s = match getter.output() {
        Ok(res) => String::from_utf8(res.stdout).unwrap(),
        Err(_) => return Err("error ocured reading data".to_owned()),
    };

    let splitted = s.split_ascii_whitespace().collect::<Vec<&str>>();
    // Freq MHz      IRQ RC6     Power W     IMC MiB/s           RCS/0           BCS/0           VCS/0          VECS/0
    // req  act       /s   %   gpu   pkg     rd     wr       %  se  wa       %  se  wa       %  se  wa       %  se  wa
    //   0    0        0   0  0.00  0.00   9072   6601   92.31   0   0    0.00   0   0    0.00   0   0    0.00   0   0
    // |    |       |      |  |     |       |       |       |   |    |      |    |   |      |    |    |     |    |   |
    // 0    1       2      3  4     5       6       7       8   9   10      11  12   13     14   15   16    17   18  19
    let freqency = splitted[1].to_owned();
    let power = splitted[5].to_owned();
    let imc_rd = splitted[6].to_owned();
    let imc_wr = splitted[7].to_owned();
    let rcs_usg = splitted[8].to_owned();
    let bcs_usg = splitted[11].to_owned();
    let vcs_usg = splitted[14].to_owned();
    let vecs_usg = splitted[17].to_owned();
    Ok(IgData {
        freqency,
        power,
        imc_rd,
        imc_wr,
        rcs_usg,
        bcs_usg,
        vcs_usg,
        vecs_usg
    })
}
