use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum Order {
    Less,
    Greater,
}

impl std::str::FromStr for Order {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Self::Less),
            ">" => Ok(Self::Greater),
            _ => Err(format!("Invalid ordering {}", s)),
        }
    }
}

impl PartialEq<Ordering> for Order {
    fn eq(&self, other: &Ordering) -> bool {
        match (self, other) {
            (Self::Less, Ordering::Less) => true,
            (Self::Greater, Ordering::Greater) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
enum Then {
    Next(u16),
    Accepted,
    Rejected,
}

enum RangeApplyResult {
    None,
    All(Then),
    Split(([(u16, u16); 4], Then), [(u16, u16); 4]),
}

#[derive(Debug)]
enum Rule {
    If(Category, Order, u16, Then),
    Else(Then),
}

impl Rule {
    fn apply_rule(&self, p: &[u16; 4]) -> Option<Then> {
        match self {
            Rule::Else(t) => Some(*t),
            Rule::If(c, o, v, t) => {
                if *o == p[*c as usize].cmp(v) {
                    Some(*t)
                } else {
                    None
                }
            }
        }
    }

    fn apply_rule_to_parts(&self, parts: &[(u16, u16); 4]) -> RangeApplyResult {
        match self {
            Rule::Else(t) => RangeApplyResult::All(*t),
            Rule::If(c, o, v, t) => {
                let r = parts[*c as usize];
                match (o, r.0.cmp(&v), r.1.cmp(&v)) {
                    (_, Ordering::Greater, _)
                    | (_, _, Ordering::Less)
                    | (_, Ordering::Equal, Ordering::Equal) => RangeApplyResult::None,
                    (Order::Greater, _, Ordering::Equal) => RangeApplyResult::None,
                    (Order::Less, Ordering::Equal, _) => RangeApplyResult::None,
                    (Order::Less, Ordering::Less, Ordering::Greater)
                    | (Order::Less, _, Ordering::Equal) => {
                        let left = {
                            let mut ranges = parts.clone();
                            ranges[*c as usize] = (r.0, v - 1);
                            ranges
                        };
                        let right = {
                            let mut ranges = parts.clone();
                            ranges[*c as usize] = (*v, r.1);
                            ranges
                        };

                        RangeApplyResult::Split((left, *t), right)
                    }
                    (_, Ordering::Less, Ordering::Greater) | (_, Ordering::Equal, _) => {
                        let left = {
                            let mut ranges = parts.clone();
                            ranges[*c as usize] = (r.0, *v);
                            ranges
                        };
                        let right = {
                            let mut ranges = parts.clone();
                            ranges[*c as usize] = (v + 1, r.1);
                            ranges
                        };

                        RangeApplyResult::Split((right, *t), left)
                    }
                }
            }
        }
    }
}

struct Aplenty {
    workflows: Vec<(u16, Vec<Rule>)>,
    parts: Vec<[u16; 4]>,
}

fn parse_id(s: &str) -> u16 {
    u16::from_str_radix(s, 36).expect(format!("Unable to parse id {}", s).as_str())
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    let rules = rules
        .strip_prefix("{")
        .and_then(|s| s.strip_suffix("}"))
        .expect("Invalid rule");
    rules
        .split(",")
        .map(|rule| match rule.find(":") {
            Some(mid) => {
                let (condition, next_id) = rule.split_at(mid);
                let then = next_id.split_at(1).1;
                let then = match then {
                    "A" => Then::Accepted,
                    "R" => Then::Rejected,
                    _ => Then::Next(parse_id(then)),
                };

                let (mid, ordering) = condition
                    .find("<")
                    .and_then(|mid| Some((mid, Order::Less)))
                    .or_else(|| {
                        condition
                            .find(">")
                            .and_then(|mid| Some((mid, Order::Greater)))
                    })
                    .expect("Unexpected ordering");

                let (category, v) = condition.split_at(mid);
                let value = v
                    .split_at(1)
                    .1
                    .parse::<u16>()
                    .expect("Could not parse value");
                let category = match category {
                    "x" => Category::X,
                    "m" => Category::M,
                    "a" => Category::A,
                    "s" => Category::S,
                    _ => panic!("Bad category"),
                };

                Rule::If(category, ordering, value, then)
            }
            None => Rule::Else(match rule {
                "A" => Then::Accepted,
                "R" => Then::Rejected,
                _ => Then::Next(parse_id(rule)),
            }),
        })
        .collect::<Vec<_>>()
}

impl Aplenty {
    fn from_input(input: &str) -> Self {
        let mut workflows = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|s| {
                let rule_start = s.find("{").expect("Invalid rule");
                let (id, rules) = s.split_at(rule_start);
                let id = parse_id(id);
                let rules = parse_rules(rules);
                (id, rules)
            })
            .collect::<Vec<_>>();

        workflows.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));

        let parts = input
            .lines()
            .skip_while(|&line| line.get(0..1).map(|s| s != "{").unwrap_or(true))
            .map(|part| {
                let part = part
                    .strip_prefix("{")
                    .and_then(|p| p.strip_suffix("}"))
                    .unwrap();

                let mut it = part.split(",");
                let x = it
                    .next()
                    .map(|s| s.split_at(2).1.parse::<u16>().unwrap())
                    .unwrap();
                let m = it
                    .next()
                    .map(|s| s.split_at(2).1.parse::<u16>().unwrap())
                    .unwrap();
                let a = it
                    .next()
                    .map(|s| s.split_at(2).1.parse::<u16>().unwrap())
                    .unwrap();
                let s = it
                    .next()
                    .map(|s| s.split_at(2).1.parse::<u16>().unwrap())
                    .unwrap();

                [x, m, a, s]
            })
            .collect::<Vec<_>>();

        Self { workflows, parts }
    }

    fn filter_parts(&self) -> u128 {
        let start_workflow = self
            .workflows
            .binary_search_by(|(id, _)| id.cmp(&parse_id("in")))
            .map(|i| &self.workflows[i].1)
            .expect("No starting point");
        self.parts
            .iter()
            .filter(|p| {
                let mut cur = start_workflow;
                loop {
                    match cur
                        .iter()
                        .find_map(|r| r.apply_rule(p))
                        .expect("No matching rule")
                    {
                        Then::Accepted => return true,
                        Then::Rejected => return false,
                        Then::Next(id) => {
                            cur = self
                                .workflows
                                .binary_search_by(|(i, _)| i.cmp(&id))
                                .map(|i| &self.workflows[i].1)
                                .expect("Next workflow does not exist");
                        }
                    }
                }
            })
            .fold(0u128, |acc, p| acc + (p[0] + p[1] + p[2] + p[3]) as u128)
    }

    fn acceptable_combinations(&self, mut part_ranges: [(u16, u16); 4], workflow_id: u16) -> u128 {
        let workflow = self
            .workflows
            .binary_search_by(|(id, _)| id.cmp(&workflow_id))
            .map(|i| &self.workflows[i].1)
            .expect("No starting point");

        let mut combinations = 0u128;
        for rule in workflow {
            match rule.apply_rule_to_parts(&part_ranges) {
                RangeApplyResult::None => (),
                RangeApplyResult::All(t) => {
                    combinations += match t {
                        Then::Rejected => 0,
                        Then::Accepted => part_ranges
                            .iter()
                            .fold(1, |c, (a, b)| c * (b - a + 1) as u128),
                        Then::Next(next_id) => self.acceptable_combinations(part_ranges, next_id),
                    };
                    break;
                }
                RangeApplyResult::Split((r0, t), r1) => {
                    part_ranges = r1;
                    combinations += match t {
                        Then::Rejected => 0,
                        Then::Accepted => r0.iter().fold(1, |c, (a, b)| c * (b - a + 1) as u128),
                        Then::Next(next_id) => self.acceptable_combinations(r0, next_id),
                    };
                }
            };
        }

        combinations
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
        let a = Aplenty::from_input(&self.0);
        let res = a.filter_parts();

        Ok(crate::AOCResult::U128(res))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let a = Aplenty::from_input(&self.0);
        let res = a.acceptable_combinations([(1, 4000); 4], parse_id("in"));

        Ok(crate::AOCResult::U128(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part_one() {
        let a = Aplenty::from_input(SAMPLE_INPUT);
        let res = a.filter_parts();
        assert_eq!(res, 19114);
    }

    #[test]
    fn test_part_two() {
        let a = Aplenty::from_input(SAMPLE_INPUT);
        let res = a.acceptable_combinations([(1, 4000); 4], parse_id("in"));
        assert_eq!(res, 167409079868000);
    }
}
