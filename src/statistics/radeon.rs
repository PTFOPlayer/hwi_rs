pub fn get_radeon() -> Result<(), ()> {
    let file = std::fs::read_to_string("./radeon");
    match file {
        Ok(res) => {
            if res.contains("err") {
                return Err(());
            }
        }
        Err(_) => return Err(()),
    }
    Err(())
}
