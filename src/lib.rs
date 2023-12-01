pub mod setup {
    pub fn read_file(path: &String) -> Result<String, Box<dyn std::error::Error>> {
        let res = std::fs::read_to_string(&path)?;
        Ok(res)
    }
}

pub mod day_one;