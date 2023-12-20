use std::str::FromStr;

#[derive(Debug)]
enum Element {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct ParseKeyError;

impl FromStr for Element {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Element::Seed),
            "soil" => Ok(Element::Soil),
            "fertilizer" => Ok(Element::Fertilizer),
            "water" => Ok(Element::Water),
            "light" => Ok(Element::Light),
            "temperature" => Ok(Element::Temperature),
            "humidity" => Ok(Element::Humidity),
            "location" => Ok(Element::Location),
            _ => Err(ParseKeyError),
        }
    }
}

#[derive(Debug)]
struct ParsingMappingError;

#[derive(Debug)]
struct Mapping {
    source: Element,
}

impl FromStr for Mapping {
    type Err = ParsingMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let get_elements = |s: &str| {
            let title = s.strip_suffix(" map:")?;
            let mut elements = title.split("-to-");
            let left = elements.next().map(|s| s.parse::<Element>().ok())??;
            let right = elements.next().map(|s| s.parse::<Element>().ok())??;

            Some((left, right))
        };

        match get_elements(&s) {
            Some(a) => Ok(Mapping { source: a.0 }),
            None => Err(ParsingMappingError),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl Rule {
    fn new(destination_start: usize, source_start: usize, length: usize) -> Self {
        Self {
            destination_start,
            source_start,
            length,
        }
    }

    fn apply(&self, source_number: usize) -> Option<usize> {
        if source_number >= self.source_start && source_number < self.source_start + self.length {
            if self.source_start < self.destination_start {
                Some(source_number + (self.destination_start - self.source_start))
            } else if self.source_start > self.destination_start {
                Some(source_number - (self.source_start - self.destination_start))
            } else {
                Some(source_number)
            }
        } else {
            None
        }
    }
}

struct ParseRuleError;

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let get_range = |line: &str| -> Option<(usize, usize, usize)> {
            let mut range_iter = line
                .split_whitespace()
                .filter_map(|w| w.parse::<usize>().ok());
            match (range_iter.next(), range_iter.next(), range_iter.next()) {
                (Some(destination_start), Some(source_start), Some(length)) => {
                    Some((destination_start, source_start, length))
                }
                _ => None,
            }
        };

        match get_range(&s) {
            Some((destination_start, source_start, length)) => {
                Ok(Rule::new(destination_start, source_start, length))
            }
            _ => Err(ParseRuleError),
        }
    }
}

#[derive(Debug)]
struct Almanac([Vec<Rule>; 7]);

impl Almanac {
    fn new() -> Self {
        Almanac(std::array::from_fn(|_| Vec::new()))
    }

    fn set_rule(&mut self, source: &Element, rule: &Rule) {
        let element_index = match source {
            Element::Seed => 0,
            Element::Soil => 1,
            Element::Fertilizer => 2,
            Element::Water => 3,
            Element::Light => 4,
            Element::Temperature => 5,
            Element::Humidity => 6,
            Element::Location => panic!("Location as source is not expected"),
        };

        self.0[element_index].push(*rule);
    }

    fn seed_info(&self, seed_number: usize) -> [usize; 7] {
        let mut res = [0; 7];

        let mut current_element_number = seed_number;
        for element_index in 0..7 {
            let rules = &self.0[element_index];
            let n = match rules
                .iter()
                .find_map(|rule| rule.apply(current_element_number))
            {
                Some(n) => n,
                _ => current_element_number,
            };

            current_element_number = n;
            res[element_index] = n;
        }

        res
    }
}

#[derive(Debug)]
pub struct Puzzle {
    almanac: Almanac,
    seeds: Vec<usize>,
}

impl Puzzle {
    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }

    fn new(input: &str) -> Self {
        let (almanac, seeds) = Self::parse_input(&input);

        Self { almanac, seeds }
    }

    fn parse_input(input: &str) -> (Almanac, Vec<usize>) {
        let mut lines = input.lines();
        let seeds = lines
            .next()
            .and_then(|s| s.strip_prefix("seeds: "))
            .map(|s| {
                s.split_whitespace()
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .unwrap_or(vec![]);
        let lines = lines.skip(1);

        let a = lines.fold(
            (Almanac::new(), Element::Seed),
            |(mut almanac, source), line| {
                if let Ok(m) = line.parse::<Mapping>() {
                    (almanac, m.source)
                } else {
                    match line.parse::<Rule>() {
                        Ok(rule) => almanac.set_rule(&source, &rule),
                        _ => (),
                    }
                    (almanac, source)
                }
            },
        );

        (a.0, seeds)
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let almanac = &self.almanac;
        let seeds = &self.seeds;
        let all_seed_info = seeds
            .iter()
            .map(|s| almanac.seed_info(*s)[6])
            .collect::<Vec<usize>>();
        let min_location = *all_seed_info.iter().min().unwrap();
        Ok(super::AOCResult::ULong(min_location as u128))
    }
    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let min = self
            .seeds
            .windows(2)
            .step_by(2)
            .filter_map(|window| {
                if let [start, length] = &window {
                    let range = *start..(*start + *length);
                    let all_seed_info = range
                        .map(|s| self.almanac.seed_info(s)[6])
                        .collect::<Vec<usize>>();
                    let min_location = all_seed_info.iter().min().unwrap();
                    Some(*min_location)
                } else {
                    None
                }
            })
            .min();

        min.map(|r| super::AOCResult::ULong(r as u128))
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No min found",
            )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn parse_input_test() {
        let (almanac, seeds) = Puzzle::parse_input(&SAMPLE_INPUT);
        let all_seed_info = seeds
            .iter()
            .map(|s| almanac.seed_info(*s))
            .collect::<Vec<[usize; 7]>>();
        let min_location = all_seed_info.iter().min_by(|x, y| x[6].cmp(&y[6])).unwrap();
        assert_eq!(min_location[6], 35);

        let min = seeds
            .windows(2)
            .step_by(2)
            .filter_map(|window| {
                if let [start, length] = &window {
                    let range = *start..(*start + *length);
                    dbg!(&range);
                    let all_seed_info = range
                        .map(|s| almanac.seed_info(s)[6])
                        .collect::<Vec<usize>>();
                    let min_location = all_seed_info.iter().min().unwrap();
                    Some(*min_location)
                } else {
                    None
                }
            })
            .min();

        assert_eq!(min.unwrap(), 35);
    }

    #[test]
    fn take_while_test() {}
}
