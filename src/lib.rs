pub mod setup {
    pub fn read_file(path: &String) -> Result<String, Box<dyn std::error::Error>> {
        let res = std::fs::read_to_string(&path)?;
        Ok(res)
    }

    pub struct Args {
        pub day: usize,
        pub puzzle_input: String,
    }
    
    pub fn parse_args(args: &[String]) -> Result<Args, Box<dyn std::error::Error>> {
        let day = args[1].clone().parse::<usize>()?;
        let file_path = args[2].clone();

        let puzzle_input = read_file(&file_path)?;
        Ok(Args{day, puzzle_input})
    }
}

pub mod day_one;