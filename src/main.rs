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
        d => {
            eprintln!("Day not implemented: {}", d);
            None
        }
    } {
        PuzzleRunner::run(puzzle);
    }
    Ok(())
}
