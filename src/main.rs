use advent_of_code_2023::day_01;
use advent_of_code_2023::day_02;
use advent_of_code_2023::day_03;
use advent_of_code_2023::day_04;
use advent_of_code_2023::setup::*;
use advent_of_code_2023::PuzzleRunner;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(puzzle) = match parse_args(&args) {
        Ok(Args { day, puzzle_input }) if day == 1 => Some(day_01::Puzzle::create(puzzle_input)),
        Ok(Args { day, puzzle_input }) if day == 2 => Some(day_02::Puzzle::create(puzzle_input)),
        Ok(Args { day, puzzle_input }) if day == 3 => Some(day_03::Puzzle::create(puzzle_input)),
        Ok(Args { day, puzzle_input }) if day == 4 => Some(day_04::Puzzle::create(puzzle_input)),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
        Ok(a) => {
            eprintln!("Day not implemented: {}", a.day);
            None
        }
    } {
        PuzzleRunner::run(puzzle);
    }
}
