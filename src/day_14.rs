use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Cube,
}

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
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

    fn key(&self) -> String {
        self.rocks
            .iter()
            .map(|r| match r {
                Some(Rock::Round) => 'O',
                Some(Rock::Cube) => '#',
                None => '.',
            })
            .collect::<String>()
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

    fn tilt(&mut self, direction: Direction) -> () {
        let mut v = vec![None; self.size * self.size];
        for i in 0..self.size {
            let mut list = match direction {
                Direction::North | Direction::South => self
                    .rocks
                    .iter()
                    .skip(i)
                    .step_by(self.size)
                    .collect::<Vec<_>>(),
                Direction::West | Direction::East => self
                    .rocks
                    .iter()
                    .skip(i * self.size)
                    .take(self.size)
                    .collect::<Vec<_>>(),
            };
            if direction == Direction::South || direction == Direction::East {
                list.reverse();
            }

            let mut last_insert_pos = 0;
            let mut foo = list
                .iter()
                .enumerate()
                .fold(vec![None; list.len()], |mut acc, a| {
                    match a.1 {
                        Some(Rock::Round) => {
                            acc[last_insert_pos] = Some(Rock::Round);
                            last_insert_pos += 1;
                        }
                        Some(Rock::Cube) => {
                            acc[a.0] = Some(Rock::Cube);
                            last_insert_pos = a.0 + 1;
                        }
                        _ => (),
                    };

                    acc
                });

            if direction == Direction::South || direction == Direction::East {
                foo.reverse();
            }

            match direction {
                Direction::North | Direction::South => {
                    foo.iter().enumerate().for_each(|(index, r)| {
                        v[index * self.size + i] = r.clone();
                    });
                }
                Direction::West | Direction::East => {
                    v.splice((i * self.size)..((i + 1) * self.size), foo);
                }
            };
        }

        self.rocks = v;
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
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
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let mut platform = Platform::from_input(&self.0);
        platform.tilt(Direction::North);

        let load = platform.calculate_load();
        Ok(super::AOCResult::ULong(load as u128))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let mut cache: std::collections::HashMap<String, (_, usize)> =
            std::collections::HashMap::new();
        let mut platform = Platform::from_input(&self.0);
        cache.insert(platform.key(), (platform.rocks.clone(), 0));
        let mut res: Option<usize> = None;
        loop {
            platform.cycle();
            let cycle_number = cache.len();

            if let Some(m) = cache.get(&platform.key()) {
                let k = cycle_number - m.1;
                let goal = ((1_000_000_000 - m.1) % k) + m.1;
                if let Some(v_n) = cache.iter().find_map(|(_, value)| {
                    if value.1 == goal {
                        Some(value.0.to_vec())
                    } else {
                        None
                    }
                }) {
                    let goal = Platform {
                        rocks: v_n,
                        size: platform.size,
                    };
                    res = Some(goal.calculate_load());
                }

                break;
            } else {
                cache.insert(platform.key(), (platform.rocks.clone(), cycle_number));
            }

            if cycle_number == 1_000_000_000 {
                break;
            }
        }
        res.map(|v| super::AOCResult::ULong(v as u128))
            .ok_or("Error".into())
    }
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
        platform.tilt(Direction::North);

        assert_eq!(platform.calculate_load(), 136);
    }

    #[test]
    fn test_cycle() {
        let mut cache: std::collections::HashMap<_, usize> = std::collections::HashMap::new();
        let mut platform = Platform::from_input(SAMPLE_INPUT);
        cache.insert(platform.rocks.clone(), 0);
        let mut res: Option<usize> = None;
        loop {
            platform.cycle();
            let cycle_number = cache.len();

            if let Some(m) = cache.get(&platform.rocks) {
                let k = cycle_number - m;
                let goal = ((1_000_000_000 - m) % k) + m;
                if let Some(v_n) = cache.iter().find(|(_, value)| *value == &goal) {
                    let goal = Platform {
                        rocks: v_n.0.to_vec(),
                        size: platform.size,
                    };
                    res = Some(goal.calculate_load());
                }

                break;
            } else {
                cache.insert(platform.rocks.clone(), cycle_number);
            }

            if cycle_number == 1_000_000_000 {
                break;
            }
        }
        assert_eq!(res.unwrap(), 64);
    }
}
