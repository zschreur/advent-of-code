pub mod grid {
    #[derive(Debug)]
    pub struct Grid<T> {
        size: usize,
        blocks: Vec<Vec<T>>,
    }

    impl<T> Grid<T> {
        pub fn new(size: usize, blocks: Vec<Vec<T>>) -> Self {
            Self { size, blocks }
        }

        pub fn size(&self) -> usize {
            self.size
        }

        pub fn get(&self, point: Point) -> Option<&T> {
            self.blocks.get(point.1).and_then(|row| row.get(point.0))
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, std::cmp::PartialOrd, std::cmp::Ord)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, std::cmp::PartialOrd, std::cmp::Ord)]
    pub struct Point(pub usize, pub usize);

    impl Point {
        pub fn new(x: usize, y: usize) -> Self {
            Self(x, y)
        }

        pub fn move_direction(&self, dir: Direction) -> Option<Self> {
            let pos = match dir {
                Direction::Up => Point(self.0, self.1.checked_sub(1)?),
                Direction::Down => Point(self.0, self.1 + 1),
                Direction::Left => Point(self.0.checked_sub(1)?, self.1),
                Direction::Right => Point(self.0 + 1, self.1),
            };

            Some(pos)
        }
    }
}

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
    U128(u128),
    U64(u64),
    U32(u32),
    USize(usize),
    I32(i32),
}

impl std::fmt::Display for AOCResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AOCResult::U128(n) => write!(f, "{}", n),
            AOCResult::U64(n) => write!(f, "{}", n),
            AOCResult::U32(n) => write!(f, "{}", n),
            AOCResult::USize(n) => write!(f, "{}", n),
            AOCResult::I32(n) => write!(f, "{}", n),
        }
    }
}

pub trait Puzzle {
    fn run_part_one(&self) -> Result<AOCResult, Box<dyn std::error::Error>>;
    fn run_part_two(&self) -> Result<AOCResult, Box<dyn std::error::Error>>;
}

impl dyn Puzzle {
    pub fn run(&self) {
        use std::time::Instant;
        let t0 = Instant::now();
        let mut total_time: std::time::Duration = std::time::Duration::ZERO;
        match self.run_part_one() {
            Ok(res) => {
                let t0_elapsed = t0.elapsed();
                total_time += t0_elapsed;
                println!("Part 1: {}", res);
                println!("Part 1 took: {:.2?}", t0_elapsed);
            }
            Err(e) => println!("Part 1 failed: {}", e),
        }
        let t1 = Instant::now();
        match self.run_part_two() {
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
pub mod day_17;
pub mod day_18;
pub mod day_19;
