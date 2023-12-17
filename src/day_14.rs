use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
}

pub struct Platform {
    size: usize,
    rocks: Vec<Option<Rock>>,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rocks.iter().enumerate().try_for_each(|(index, rock)| {
            if index % self.size == 0 && index != 0 {
                writeln!(f)?;
            }

            write!(
                f,
                "{}",
                match rock {
                    Some(Rock::Round) => 'O',
                    Some(Rock::Cube) => '#',
                    None => '.',
                }
            )
        })
    }
}

impl Platform {
    fn new(rocks: Vec<Option<Rock>>, size: usize) -> Self {
        Self { rocks, size }
    }

    fn from_input(input: &str) -> Self {
        let size = input.lines().count();
        let rocks = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => None,
                        'O' => Some(Rock::Round),
                        '#' => Some(Rock::Cube),
                        _ => panic!("Invalid input"),
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        Self::new(rocks, size)
    }

    fn tilt(&mut self) -> () {
        let mut v = vec![None; self.size * self.size];
        for x in 0..self.size {
            self.rocks
                .iter()
                .skip(x)
                .step_by(self.size)
                .collect::<Vec<_>>()
                .split_inclusive_mut(|r| match r {
                    Some(Rock::Cube) => true,
                    _ => false,
                })
                .map(|slice| {
                    slice.sort_by(|a, b| match (a, b) {
                        (Some(Rock::Round), None) => std::cmp::Ordering::Less,
                        (None, Some(Rock::Round)) => std::cmp::Ordering::Greater,
                        (Some(Rock::Cube), _) => std::cmp::Ordering::Greater,
                        (_, Some(Rock::Cube)) => std::cmp::Ordering::Less,
                        _ => std::cmp::Ordering::Equal,
                    });
                    slice
                })
                .flatten()
                .enumerate()
                .for_each(|(index, r)| {
                    v[index * self.size + x] = r.clone();
                });
        }

        self.rocks = v;
    }

    fn calculate_load(&self) -> usize {
        self.rocks.iter().enumerate().fold(0, |acc, (index, rock)| {
            if rock == &Some(Rock::Round) {
                let load = self.size - (index / self.size);
                acc + load
            } else {
                acc
            }
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
        let mut platform = Platform::from_input(&self.0);
        platform.tilt();

        let load = platform.calculate_load();
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
        assert_eq!(platform.rocks.len(), 100);
    }

    #[test]
    fn test_tilt() {
        let mut platform = Platform::from_input(SAMPLE_INPUT);

        println!("{}", platform);
        platform.tilt();
        println!("\n{}", platform);

        assert_eq!(platform.calculate_load(), 136);
    }
}
