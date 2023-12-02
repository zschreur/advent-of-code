use advent_of_code_2023::day_one;
use advent_of_code_2023::setup::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match parse_args(&args) {
        Ok(Args { day, puzzle_input }) if day == 1 => {
            day_one::run(&puzzle_input);
        },
        Err(e) => eprintln!("{}", e),
        Ok(a) => eprintln!("Day not implemented: {}", a.day),
    }
}
