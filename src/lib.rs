pub mod setup {
    pub struct Args {
        pub day: usize,
        pub puzzle_input: String,
    }

    pub fn parse_args(args: &[String]) -> Result<Args, Box<dyn std::error::Error>> {
        let day = args[1].clone().parse::<usize>()?;

        let puzzle_input_path = format!("./input/day-{:0>2}.txt", day);
        let puzzle_input_path = std::path::Path::new(&puzzle_input_path);
        match std::fs::read_to_string(puzzle_input_path) {
            Ok(puzzle_input) => Ok(Args { day, puzzle_input }),
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub enum AOCResult {
    ULong(u128),
    Long(i128),
}

impl std::fmt::Display for AOCResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AOCResult::ULong(n) => write!(f, "{}", n),
            AOCResult::Long(n) => write!(f, "{}", n),
        }
    }
}

pub trait Puzzle {
    fn run_part_one(&self) -> Result<AOCResult, Box<dyn std::error::Error>>;
    fn run_part_two(&self) -> Result<AOCResult, Box<dyn std::error::Error>>;
}

pub struct PuzzleRunner {}

impl PuzzleRunner {
    pub fn run(puzzle: Box<dyn Puzzle>) {
        use std::time::Instant;
        let t0 = Instant::now();
        let mut total_time: std::time::Duration = std::time::Duration::ZERO;
        match puzzle.run_part_one() {
            Ok(res) => {
                let t0_elapsed = t0.elapsed();
                total_time += t0_elapsed;
                println!("Part 1: {}", res);
                println!("Part 1 took: {:.2?}", t0_elapsed);
            }
            Err(e) => println!("Part 1 failed: {}", e),
        }
        let t1 = Instant::now();
        match puzzle.run_part_two() {
            Ok(res) => {
                let t1_elapsed = t1.elapsed();
                total_time += t1_elapsed;
                println!("Part 2: {}", res);
                println!("Part 2 took: {:.2?}", t1_elapsed);
            }
            Err(e) => println!("Part 2 failed: {}", e),
        }
        println!("Total time: {:.2?}", total_time);
    }
}

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
