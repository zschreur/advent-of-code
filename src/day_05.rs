use std::collections::HashMap;
use std::str::FromStr;

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

struct Mapping {
    source: Element,
    destination: Element,
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
            Some(a) => Ok(Mapping {
                source: a.0,
                destination: a.1,
            }),
            None => Err(ParsingMappingError),
        }
    }
}

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
            Some((destination_start, source_start, length)) => Ok(Rule::new(destination_start, source_start, length)),
            _ => Err(ParseRuleError)

        }
    }
}

pub struct Puzzle;

impl Puzzle {
    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self)
    }

    fn parse_input(input: &str) -> () {
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

        lines.fold(0, |acc, line| {
            if let Ok(m) = line.parse::<Mapping>() {
            } else if let Ok(rule) = line.parse::<Rule>() {
            } else {
            }

            acc
        });
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {
        println!("Part 1: {}", "NOT IMPLEMENTED");
    }
    fn run_part_two(&self) {
        println!("Part 2: {}", "NOT IMPLEMENTED");
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
        // Puzzle::parse_input(SAMPLE_INPUT);
        assert!(false);
    }

    #[test]
    fn take_while_test() {
        let input = "seed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15";
    }
}
