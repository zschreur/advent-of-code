#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
}

pub struct Platform {
    size: usize,
    rocks: Vec<Vec<Option<Rock>>>,
}

impl Platform {
    fn new(size: usize) -> Self {
        Self {
            size,
            rocks: vec![vec![None; size]; size],
        }
    }

    fn from_input(input: &str) -> Self {
        let size = input.lines().count();
        let mut platform = Self::new(size);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    'O' => platform.rocks[y][x] = Some(Rock::Round),
                    '#' => platform.rocks[y][x] = Some(Rock::Cube),
                    _ => panic!("Invalid input"),
                }
            }
        }

        platform
    }

    fn tilt(&self) -> Self {
        let mut new_platform = Self::new(self.size);

        for x in 0..self.size {
            let mut last_pos = 0;
            for y in 0..self.size {
                match self.rocks[y][x] {
                    Some(Rock::Round) => {
                        new_platform.rocks[last_pos][x] = Some(Rock::Round);
                        last_pos += 1;
                    }
                    Some(Rock::Cube) => {
                        (last_pos..y).for_each(|_| new_platform.rocks[last_pos][x] = None);
                        new_platform.rocks[y][x] = Some(Rock::Cube);
                        last_pos = y + 1;
                    }
                    None => (),
                }
            }
            (last_pos..self.size).for_each(|_| new_platform.rocks[last_pos][x] = None);
        }

        new_platform
    }

    fn calculate_load(&self) -> usize {
        self.rocks.iter().enumerate().fold(0, |acc, (index, row)| {
            acc + (row
                .iter()
                .filter(|rock| match rock {
                    Some(Rock::Round) => true,
                    _ => false,
                })
                .count()
                * (self.size - index))
        })
    }
}

pub struct Puzzle(String);

impl Puzzle {
    fn new(input: &str) -> Self {
        Self(input.to_string())
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {
        let platform = Platform::from_input(&self.0);
        let tilted = platform.tilt();

        let load = tilted.calculate_load();
        println!("Part 1: {}", load);
    }

    fn run_part_two(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_parse_input() {
        let platform = Platform::from_input(SAMPLE_INPUT);

        assert_eq!(platform.size, 10);
        assert_eq!(
            platform.rocks[0],
            vec![
                Some(Rock::Round),
                None,
                None,
                None,
                None,
                Some(Rock::Cube),
                None,
                None,
                None,
                None
            ]
        );

        assert_eq!(
            platform.rocks[1],
            vec![
                Some(Rock::Round),
                None,
                Some(Rock::Round),
                Some(Rock::Round),
                Some(Rock::Cube),
                None,
                None,
                None,
                None,
                Some(Rock::Cube)
            ]
        );
    }

    #[test]
    fn test_tilt() {
        let platform = Platform::from_input(SAMPLE_INPUT);

        platform.rocks.iter().for_each(|row| {
            row.iter().for_each(|rock| {
                print!(
                    "{} ",
                    match rock {
                        Some(Rock::Round) => 'O',
                        Some(Rock::Cube) => '#',
                        None => '.',
                    }
                );
            });
            println!();
        });
        println!();

        let tilted = platform.tilt();

        tilted.rocks.iter().for_each(|row| {
            row.iter().for_each(|rock| {
                print!(
                    "{} ",
                    match rock {
                        Some(Rock::Round) => 'O',
                        Some(Rock::Cube) => '#',
                        None => '.',
                    }
                );
            });
            println!();
        });

        assert_eq!(tilted.calculate_load(), 136);
    }
}
