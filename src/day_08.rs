use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

const AAA: u128 = 0;
const Z: u128 = 'Z' as u128 - 'A' as u128;
const ZZZ: u128 = (Z << 16) | (Z << 8) | Z;

#[derive(Debug)]
struct ParseMapError;
fn parse_map(input: &str) -> Result<(Vec<Direction>, HashMap<u128, (u128, u128)>), ParseMapError> {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .ok_or(ParseMapError)?
        .chars()
        .filter_map(|c| match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })
        .collect::<Vec<Direction>>();

    lines.next();
    let m = lines
        .try_fold(HashMap::new(), |mut acc, line| {
            let (source_node, children) = line.split_at(4);
            let key = source_node
                .trim()
                .chars()
                .fold(0u128, |acc, c| (acc << 8) | (c as u128 - 'A' as u128));
            let children = children.strip_prefix("= ")?;
            let left = children
                .get(1..=3)?
                .chars()
                .fold(0u128, |acc, c| (acc << 8) | (c as u128 - 'A' as u128));
            let right = children
                .get(6..=8)?
                .chars()
                .fold(0u128, |acc, c| (acc << 8) | (c as u128 - 'A' as u128));

            acc.insert(key, (left, right));

            Some(acc)
        })
        .ok_or(ParseMapError)?;

    Ok((directions, m))
}

#[derive(Debug)]
struct SearchError;
fn steps_to_reach_node(
    start_node: u128,
    directions: &Vec<Direction>,
    map: &HashMap<u128, (u128, u128)>,
    cmp: fn(u128) -> bool,
) -> Result<u128, SearchError> {
    let mut current = map.get(&start_node).ok_or(SearchError)?;
    let res = directions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(index, direction)| {
            let next = match direction {
                Direction::Left => current.0,
                _ => current.1,
            };

            if cmp(next) {
                Some(index)
            } else {
                current = map.get(&next)?;
                None
            }
        })
        .ok_or(SearchError)?;

    Ok(res as u128 + 1)
}

#[derive(Debug)]
struct PartOneError;
fn run_part_one(
    directions: Vec<Direction>,
    map: HashMap<u128, (u128, u128)>,
) -> Result<u128, PartOneError> {
    let cmp = |v: u128| v == ZZZ;
    let res = steps_to_reach_node(AAA, &directions, &map, cmp).map_err(|_| PartOneError)?;

    Ok(res)
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

fn gcd(a: u128, nums: &[u128]) -> u128 {
    if let Some(&b) = nums.first() {
        let c = (1..=a.min(b))
            .rev()
            .find(|v| {
                a % v == 0 && b % v == 0
            })
            .unwrap_or(1);
        gcd(c, nums.split_at(1).1)
    } else {
        a
    }
}

fn lcm(a: u128, nums: &[u128]) -> u128 {
    if let Some(&b) = nums.first() {
        let foo = gcd(a, &[b]);
        let c = a * (b / foo);
        lcm(c, nums.split_at(1).1)
    } else {
        a
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {
        let (directions, map) = parse_map(&self.0).expect("Issue parsing input");
        let res = run_part_one(directions, map).expect("Issue running part one");
        println!("Part 1: {}", res);
    }

    fn run_part_two(&self) {
        let (directions, map) = parse_map(&self.0).expect("Issue parsing input");
        let starting_nodes = map
            .keys()
            .filter(|&k| (k & 0xFF as u128) == 0)
            .collect::<Vec<&u128>>();
        let counts = starting_nodes
            .iter()
            .filter_map(|&k| {
                steps_to_reach_node(*k, &directions, &map, |v| (v & 0xFF as u128) == Z)
                    .map_err(|_| PartOneError)
                    .ok()
            })
            .collect::<Vec<u128>>();

        let a = counts.first().unwrap();
        let b = counts.split_at(1).1;
        let res = lcm(*a, &b);
        println!("Part 2: {}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ";

    #[test]
    fn test() {
        let (a, b) = parse_map(&SAMPLE_INPUT).unwrap();
        assert_eq!(run_part_one(a, b).unwrap(), 6);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, &vec![3]), 6);
        assert_eq!(lcm(21, &vec![6]), 42);
        assert_eq!(lcm(1, &vec![3, 8, 10]), 120);
    }
}
