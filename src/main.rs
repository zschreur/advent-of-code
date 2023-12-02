use advent_of_code_2023::day_01;
use advent_of_code_2023::day_02;
use advent_of_code_2023::day_03;
use advent_of_code_2023::setup::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match parse_args(&args) {
        Ok(Args { day, puzzle_input }) if day == 1 => {
            day_01::run(&puzzle_input);
        }
        Ok(Args { day, puzzle_input }) if day == 2 => {
            day_02::run(&puzzle_input);
        }
        Ok(Args { day, puzzle_input }) if day == 3 => {
            day_03::run(&puzzle_input);
        }
        Err(e) => eprintln!("{}", e),
        Ok(a) => eprintln!("Day not implemented: {}", a.day),
    }
}

