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
            Ok(puzzle_input) => Ok(Args {
                day,
                puzzle_input,
            }),
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub trait Puzzle {
    fn run_part_one(&self);
    fn run_part_two(&self);
}

pub struct PuzzleRunner {
}

impl PuzzleRunner {
    pub fn run(puzzle: Box<dyn Puzzle>) {
        use std::time::Instant;
        let t0 = Instant::now();
        puzzle.run_part_one();
        let t0_elapsed = t0.elapsed();
        let t1 = Instant::now();
        puzzle.run_part_two();
        let t1_elapsed = t1.elapsed();
        println!("Part 1 took: {:.2?}", t0_elapsed);
        println!("Part 2 took: {:.2?}", t1_elapsed);
        println!("Total time: {:.2?}", t0_elapsed + t1_elapsed);
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
