use std::collections::BTreeMap;

#[derive(Debug)]
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

#[derive(Debug)]
enum Rule {
    If(Category, std::cmp::Ordering, u16, Then),
    Else(Then),
}

impl Rule {
    fn apply_rule(&self, p: &(u16, u16, u16, u16)) -> Option<Then> {
        use Category::*;
        match self {
            Rule::Else(t) => Some(*t),
            Rule::If(c, o, v, t) => {
                if match c {
                    X => p.0.cmp(v) == *o,
                    M => p.1.cmp(v) == *o,
                    A => p.2.cmp(v) == *o,
                    S => p.3.cmp(v) == *o,
                } {
                    Some(*t)
                } else {
                    None
                }
            }
        }
    }
}

struct Aplenty {
    workflows: BTreeMap<u16, Vec<Rule>>,
    parts: Vec<(u16, u16, u16, u16)>,
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

                let (mid, comparison) = condition
                    .find("<")
                    .and_then(|mid| Some((mid, std::cmp::Ordering::Less)))
                    .or_else(|| {
                        condition
                            .find(">")
                            .and_then(|mid| Some((mid, std::cmp::Ordering::Greater)))
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

                Rule::If(category, comparison, value, then)
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
        let workflows = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|s| {
                let rule_start = s.find("{").expect("Invalid rule");
                let (id, rules) = s.split_at(rule_start);
                let id = parse_id(id);
                let rules = parse_rules(rules);
                (id, rules)
            })
            .collect::<BTreeMap<_, _>>();

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

                (x, m, a, s)
            })
            .collect::<Vec<_>>();

        Self { workflows, parts }
    }

    fn filter_parts(&self) -> u128 {
        let start_workflow = self
            .workflows
            .get(&parse_id("in"))
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
                                .get(&id)
                                .expect("Next workflow does not exist");
                        }
                    }
                }
            })
            .fold(0u128, |acc, p| acc + (p.0 + p.1 + p.2 + p.3) as u128)
    }

    fn acceptable_combinations(&self) -> u128 {
        todo!();
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

        Ok(crate::AOCResult::ULong(res as u128))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let a = Aplenty::from_input(&self.0);
        let res = a.acceptable_combinations();

        Ok(crate::AOCResult::ULong(res as u128))
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
        let res = a.acceptable_combinations();
        assert_eq!(res, 167409079868000);
    }
}
