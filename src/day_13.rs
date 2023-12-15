#[derive(Debug, PartialEq)]
struct Pattern(Vec<u128>, Vec<u128>);

impl Pattern {
    fn new(horizontal: Vec<u128>, vertical: Vec<u128>) -> Self {
        Self(horizontal, vertical)
    }

    fn parse(input: &str) -> Self {
        let vertical = input.lines().fold(
            vec![0; input.lines().next().unwrap_or("").len()],
            |acc, line| {
                line.chars()
                    .zip(acc)
                    .map(|(c, acc)| match c {
                        '#' => acc << 1 | 1,
                        '.' => acc << 1,
                        _ => panic!("Invalid character"),
                    })
                    .collect::<Vec<u128>>()
            },
        );
        let horizontal = input
            .lines()
            .map(|line| {
                line.chars().fold(0u128, |acc, c| match c {
                    '#' => acc << 1 | 1,
                    '.' => acc << 1,
                    _ => panic!("Invalid character"),
                })
            })
            .collect::<Vec<u128>>();
        Self::new(horizontal, vertical)
    }

    fn find_reflection(&self) -> (Option<usize>, Option<usize>) {
        let f = |v: &Vec<u128>| -> Option<usize> {
            dbg!(&v);
            match v.iter().enumerate().skip(1).find(|(index, _)| {
                let mut offset = 0;
                loop {
                    let left = v.get(index - offset - 1);
                    let right = v.get(index + offset);
                    if let (Some(left), Some(right)) = (left, right) {
                        match left ^ right {
                            0 => offset += 1,
                            _ => break,
                        }
                    } else {
                        return true;
                    }
                }

                false
            }) {
                Some((index, _)) => Some(index),
                _ => None,
            }
        };

        if let Some(h) = f(&self.0) {
            (Some(h), None)
        } else if let Some(v) = f(&self.1) {
            (None, Some(v))
        } else {
            (None, None)
        }
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
    fn run_part_one(&self) {}

    fn run_part_two(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_parse_single_puzzle() {
        let p = Pattern::parse(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(
            p,
            Pattern::new(
                vec![
                    0b101100110,
                    0b001011010,
                    0b110000001,
                    0b110000001,
                    0b001011010,
                    0b001100110,
                    0b101011010
                ],
                vec![
                    0b1011001, 0b0011000, 0b1100111, 0b1000010, 0b0100101, 0b0100101, 0b1000010,
                    0b1100111, 0b0011000
                ]
            )
        );
    }

    #[test]
    fn test_find_reflection() {
        let p = Pattern::parse(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );

        assert_eq!(p.find_reflection(), (None, Some(5)));

        let p = Pattern::parse(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );

        assert_eq!(p.find_reflection(), (Some(4), None));
    }
}
