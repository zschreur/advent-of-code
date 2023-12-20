const NUMBERS: &'static [(&'static str, usize)] = &[
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn first_and_last_digit(line: &str) -> Option<usize> {
    let mut res = None;
    line.chars().into_iter().for_each(|char| {
        if let Ok(v) = char.to_string().parse::<usize>() {
            res = match res {
                None => Some((v, v)),
                Some((a, _)) => Some((a, v)),
            };
        }
    });
    match res {
        Some((a, b)) => Some(a * 10 + b),
        _ => None,
    }
}

fn first_and_last_number(line: &str) -> Option<usize> {
    let first = NUMBERS
        .iter()
        .filter_map(|(s, n)| match line.find(s) {
            Some(pos) => Some((pos, n)),
            None => None,
        })
        .min_by(|x, y| x.0.cmp(&y.0));

    let last = NUMBERS
        .iter()
        .filter_map(|(s, n)| match line.rfind(s) {
            Some(pos) => Some((pos, n)),
            None => None,
        })
        .max_by(|x, y| x.0.cmp(&y.0));

    match (first, last) {
        (Some((_, a)), Some((_, b))) => Some(a * 10 + b),
        _ => None,
    }
}

fn parse_trebuchet_calibration(document: &String, parse_fn: fn(&str) -> Option<usize>) -> usize {
    document
        .lines()
        .filter_map(|line| parse_fn(line))
        .fold(0, |acc, x| acc + x)
}

pub struct Puzzle(String);

impl Puzzle {
    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self(input))
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let result = parse_trebuchet_calibration(&self.0, first_and_last_digit);
        Ok(super::AOCResult::ULong(result as u128))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let result = parse_trebuchet_calibration(&self.0, first_and_last_number);
        Ok(super::AOCResult::ULong(result as u128))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_first_and_last_digit() {
        assert_eq!(first_and_last_digit("12"), Some(12));
        assert_eq!(first_and_last_digit("a1b2c"), Some(12));
        assert_eq!(first_and_last_digit("a1b"), Some(11));
        assert_eq!(first_and_last_digit("ab"), None);
    }

    #[test]
    fn test_parse_first_and_last_number() {
        assert_eq!(first_and_last_number("a1b"), Some(11));
        assert_eq!(first_and_last_number("ab"), None);
        assert_eq!(first_and_last_number("one"), Some(11));
        assert_eq!(first_and_last_number("onetwo"), Some(12));
        assert_eq!(first_and_last_number("abonecdtwoef"), Some(12));
        assert_eq!(first_and_last_number("onetwoone"), Some(11));
        assert_eq!(first_and_last_number("onetwo1"), Some(11));
    }
}
