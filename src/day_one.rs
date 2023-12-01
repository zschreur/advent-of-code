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

fn parse_document_line(line: &str) -> Option<usize> {
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

fn parse_trebuchet_calibration(document: &String) -> usize {
    document
        .split("\n")
        .into_iter()
        .filter_map(|line| parse_document_line(line))
        .fold(0, |acc, x| acc + x)
}

pub fn run(input: &String) -> () {
    let res = parse_trebuchet_calibration(input);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_document_line("12"), Some(12));
        assert_eq!(parse_document_line("a1b2c"), Some(12));
        assert_eq!(parse_document_line("a1b"), Some(11));
        assert_eq!(parse_document_line("ab"), None);
        assert_eq!(parse_document_line("one"), Some(11));
        assert_eq!(parse_document_line("onetwo"), Some(12));
        assert_eq!(parse_document_line("abonecdtwoef"), Some(12));
        assert_eq!(parse_document_line("onetwoone"), Some(11));
    }
}
