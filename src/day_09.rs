use std::collections::HashSet;

pub struct Puzzle(Vec<Vec<i32>>);

impl Puzzle {
    fn new(input: &str) -> Self {
        let sequences = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        Self(sequences)
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {
        let res = self
            .0
            .iter()
            .map(|seq| find_next_value(&seq))
            .reduce(|acc, v| acc + v);
        println!("Part 1: {}", res.unwrap());
    }

    fn run_part_two(&self) {
        let res = self
            .0
            .iter()
            .map(|seq| {
                let mut v = seq.clone();
                v.reverse();
                find_next_value(&v)
            })
            .reduce(|acc, v| acc + v);
        println!("Part 2: {}", res.unwrap());
    }
}

fn find_next_value(sequence: &Vec<i32>) -> i32 {
    let d = sequence
        .windows(2)
        .map(|w| &w[1] - &w[0])
        .collect::<Vec<i32>>();
    let s = HashSet::<&i32>::from_iter(d.iter());

    sequence.last().unwrap()
        + if s.len() == 1 {
            d[0]
        } else {
            find_next_value(&d)
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_parse_input() {
        let p = Puzzle::new(&SAMPLE_INPUT);
        assert_eq!(p.0.len(), 3);
        assert_eq!(p.0[0], vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(p.0[1], vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(p.0[2], vec![10, 13, 16, 21, 30, 45]);
    }

    #[test]
    fn test_find_next_value() {
        assert_eq!(find_next_value(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(find_next_value(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(find_next_value(&vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_find_prev_value() {
        let mut v = vec![0, 3, 6, 9, 12, 15];
        v.reverse();
        assert_eq!(find_next_value(&v), -3);
        let mut v = vec![1, 3, 6, 10, 15, 21];
        v.reverse();
        assert_eq!(find_next_value(&v), 0);
        let mut v = vec![10, 13, 16, 21, 30, 45];
        v.reverse();
        assert_eq!(find_next_value(&v), 5);
    }
}
