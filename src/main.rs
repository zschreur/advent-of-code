use advent_of_code_2023::setup::read_file;
use advent_of_code_2023::day_one;

fn main() {
    // exec puzzle_input_path
    let args: Vec<String> = std::env::args().collect();
    let puzzle_input_path = args.get(1).unwrap();
    match read_file(&puzzle_input_path) {
        Ok(input) => {
            day_one::run(&input);
        },
        Err(e) => {
            eprint!("{}", e);
        }
    }
}
