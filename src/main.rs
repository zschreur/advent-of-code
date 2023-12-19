use advent_of_code_2023::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let args = setup::parse_args(&args)?;
    if let Some(puzzle) = match args.day {
        1 => Some(day_01::Puzzle::create(args.puzzle_input)),
        2 => Some(day_02::Puzzle::create(args.puzzle_input)),
        3 => Some(day_03::Puzzle::create(args.puzzle_input)),
        4 => Some(day_04::Puzzle::create(args.puzzle_input)),
        5 => Some(day_05::Puzzle::create(args.puzzle_input)),
        6 => Some(day_06::Puzzle::create(args.puzzle_input)),
        7 => Some(day_07::Puzzle::create(args.puzzle_input)),
        8 => Some(day_08::Puzzle::create(args.puzzle_input)),
        9 => Some(day_09::Puzzle::create(args.puzzle_input)),
        10 => Some(day_10::Puzzle::create(args.puzzle_input)),
        11 => Some(day_11::Puzzle::create(args.puzzle_input)),
        12 => Some(day_12::Puzzle::create(args.puzzle_input)),
        13 => Some(day_13::Puzzle::create(args.puzzle_input)),
        14 => Some(day_14::Puzzle::create(args.puzzle_input)),
        15 => Some(day_15::Puzzle::create(args.puzzle_input)),
        16 => Some(day_16::Puzzle::create(args.puzzle_input)),
        17 => Some(day_17::Puzzle::create(args.puzzle_input)),
        d => {
            eprintln!("Day not implemented: {}", d);
            None
        }
    } {
        PuzzleRunner::run(puzzle);
    }
    Ok(())
}
