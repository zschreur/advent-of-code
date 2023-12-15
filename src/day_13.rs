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

    fn parse_all_puzzles(input: &str) -> Vec<Self> {
        input
            .split("\n\n")
            .map(|puzzle| Self::parse(puzzle))
            .collect::<Vec<Self>>()
    }

    fn find_reflection(&self, expected_smudge_count: u32) -> (Option<usize>, Option<usize>) {
        let f = |v: &Vec<u128>| -> Option<usize> {
            match v.iter().enumerate().skip(1).find(|(index, _)| {
                let mut offset = 0;
                let mut smudge_count = 0;
                loop {
                    let left = index
                        .checked_sub(offset)
                        .and_then(|i| i.checked_sub(1))
                        .and_then(|i| v.get(i));
                    let right = v.get(index + offset);
                    if let (Some(left), Some(right)) = (left, right) {
                        smudge_count += (left ^ right).count_ones();
                        match smudge_count <= expected_smudge_count {
                            true => offset += 1,
                            _ => break,
                        }
                    } else {
                        return smudge_count == expected_smudge_count;
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
    fn run_part_one(&self) {
        let patterns = Pattern::parse_all_puzzles(&self.0);
        let res = patterns.iter().fold(0u128, |acc, pattern| {
            acc + match pattern.find_reflection(0) {
                (Some(h), None) => h as u128 * 100,
                (None, Some(v)) => v as u128,
                _ => 0,
            }
        });

        println!("Part 1: {}", res);
    }

    fn run_part_two(&self) {
        let patterns = Pattern::parse_all_puzzles(&self.0);
        let res = patterns.iter().fold(0u128, |acc, pattern| {
            acc + match pattern.find_reflection(1) {
                (Some(h), None) => h as u128 * 100,
                (None, Some(v)) => v as u128,
                _ => 0,
            }
        });

        println!("Part 2: {}", res);
    }
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
    fn test_parse_sample_input() {
        let p = Pattern::parse_all_puzzles(SAMPLE_INPUT);
        assert_eq!(p.len(), 2);
    }

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

        assert_eq!(p.find_reflection(0), (None, Some(5)));

        let p = Pattern::parse(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );

        assert_eq!(p.find_reflection(0), (Some(4), None));
    }

    #[test]
    fn test_find_reflection_with_correction() {
        let p = Pattern::parse(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );

        assert_eq!(p.find_reflection(1), (Some(3), None));

        let p = Pattern::parse(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );

        assert_eq!(p.find_reflection(1), (Some(1), None));
    }
}
