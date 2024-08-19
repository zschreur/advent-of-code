use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Condition {
    Broken,
    Unknown,
}

struct Record {
    spring_groups: Vec<Vec<Condition>>,
    code: Vec<usize>,
}

#[derive(Debug)]
struct ParseRecordError;
impl FromStr for Record {
    type Err = ParseRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, code) = {
            let mut split = s.split_whitespace();
            let spings = split.next().ok_or(ParseRecordError)?;
            let code = split.next().ok_or(ParseRecordError)?;
            (spings, code)
        };

        let code = code
            .split(',')
            .map(|c| c.parse::<usize>().expect("Issue parsing code"))
            .collect::<Vec<usize>>();

        let spring_groups = springs
            .split('.')
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '#' => Condition::Broken,
                        '?' => Condition::Unknown,
                        _ => panic!("Issue parsing spings"),
                    })
                    .collect::<Vec<Condition>>()
            })
            .collect::<Vec<Vec<Condition>>>();

        Ok(Record {
            spring_groups,
            code,
        })
    }
}

fn count_arrangements(
    groups: &[Vec<Condition>],
    blocks: &[usize],
    cache: &mut HashMap<(Vec<Vec<Condition>>, Vec<usize>), u128>,
) -> u128 {
    let group_split = groups.split_first();
    if group_split.is_none() {
        return if blocks.len() > 0 { 0 } else { 1 };
    }
    let (group, remaining_groups) = group_split.unwrap();

    let block_split = blocks.split_first();
    if block_split.is_none() {
        return if groups.iter().any(|g| g.contains(&Condition::Broken)) {
            0
        } else {
            1
        };
    }
    let (&block, remaining_blocks) = block_split.unwrap();

    if block > group.len() && group.contains(&Condition::Broken) {
        return 0;
    }

    match cache.get(&(groups.to_vec(), blocks.to_vec())) {
        Some(cached) => {
            return *cached;
        }
        None => (),
    }

    let mut possible_arrangements: u128 = 0;
    if !group.contains(&Condition::Broken) {
        possible_arrangements = count_arrangements(remaining_groups, blocks, cache);
    }

    let mut offset = 0usize;
    let mut next_calls: HashMap<Vec<Vec<Condition>>, usize> = HashMap::new();
    loop {
        if offset + block > group.len() {
            break;
        }

        match offset.checked_sub(2).and_then(|i| group.get(i)) {
            Some(Condition::Broken) => {
                break;
            }
            _ => (),
        };
        match offset.checked_sub(1).and_then(|i| group.get(i)) {
            Some(Condition::Broken) => {
                offset += 1;
                continue;
            }
            _ => (),
        };
        match group.get(offset + block) {
            Some(Condition::Broken) => {
                offset += 1;
                continue;
            }
            _ => (),
        }

        let mid = offset + block + 1;
        let next_call = if mid < group.len() {
            let (_, remainder) = group.split_at(offset + block + 1);
            let mut new_groups = remaining_groups.to_owned();
            new_groups.insert(0, remainder.to_owned());
            new_groups
        } else {
            remaining_groups.to_owned()
        };
        match next_calls.get_mut(&next_call) {
            Some(count) => {
                *count += 1;
            }
            None => {
                next_calls.insert(next_call, 1);
            }
        }

        offset += 1;
    }
    possible_arrangements += next_calls
        .iter()
        .map(|(call, c)| *c as u128 * count_arrangements(call, remaining_blocks, cache))
        .sum::<u128>();

    cache.insert((groups.to_vec(), blocks.to_vec()), possible_arrangements);
    possible_arrangements
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
        let mut cache = HashMap::new();
        let res = self
            .0
            .lines()
            .map(|l| {
                let record = l.parse::<Record>().unwrap();
                count_arrangements(&record.spring_groups, &record.code, &mut cache)
            })
            .sum::<u128>();

        Ok(super::AOCResult::U128(res))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let mut cache = HashMap::new();
        let res = self
            .0
            .lines()
            .map(|l| {
                let mut s = l.split_whitespace();
                let a = s.next().unwrap();
                let b = s.next().unwrap();
                let new_cond = format!("{}?{}?{}?{}?{}", a, a, a, a, a);
                let new_code = format!("{},{},{},{},{}", b, b, b, b, b);
                let new_line = format!("{} {}", new_cond, new_code);
                let record = new_line.parse::<Record>().unwrap();
                count_arrangements(&record.spring_groups, &record.code, &mut cache)
            })
            .sum::<u128>();

        Ok(super::AOCResult::U128(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! count_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (record, expected) = $value;
            let record = record.parse::<Record>().unwrap();
            let mut cache = HashMap::new();
            assert_eq!(expected, count_arrangements(&record.spring_groups, &record.code, &mut cache));
        }
    )*
    }
    }

    count_tests! {
        count_0: ("### 3", 1),
        count_1: ("??? 3", 1),
        count_2: ("??. 2", 1),
        count_3: ("??..??? 2,3", 1),
        count_4: ("??? 1,1", 1),
        count_5: ("???.### 1,1,3", 1),
        count_6: (".??..??...?##. 1,1,3", 4),
        count_7: ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
        count_8: ("????.#...#... 4,1,1", 1),
        count_9: ("????.######..#####. 1,6,5", 4),
        count_10: ("?###???????? 3,2,1", 10),
        count_11: ("????????#.?. 2,2,2", 3),
        count_12: ("..???..??#??.? 1,4", 6),
        count_13: ("???#####??? 1,6,1", 4),
        count_14: ("???#####????????? 1,6,1,1", 51),
        count_15: ("??.????.?..?.???. 2,1", 25),
        count_16: ("? 1", 1),
        count_17: ("?? 1", 2),
        count_18: ("# 1", 1),
        count_19: ("## 2", 1),
        count_20: ("?? 2", 1),
        count_21: ("#?? 1", 1), // AHA!
    }

    const SAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_sample_input() {
        let mut cache = HashMap::new();
        let res = SAMPLE_INPUT
            .lines()
            .map(|l| {
                let record = l.parse::<Record>().unwrap();
                count_arrangements(&record.spring_groups, &record.code, &mut cache)
            })
            .sum::<u128>();

        assert_eq!(res, 21);
    }
}
