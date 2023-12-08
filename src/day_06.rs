pub struct Puzzle(String);

impl Puzzle {
    fn new(input: &str) -> Self {
        Self(input.to_string())
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }

    fn parse_input<T>(&self, parser: fn(&str) -> T) -> T {
        parser(self.0.as_str())
    }
}

fn ways_to_win(time: u128, record: u128) -> u128 {
    let s = f64::sqrt((time.pow(2) - (4 * record)) as f64);

    let max = (time as f64 + s) / 2.;
    let max = max.ceil() as u128 - 1;
    let min = (time as f64 - s) / 2.;
    let min = min.floor() as u128 + 1;

    max - min + 1
}

#[derive(Debug)]
struct ParsingError;
impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {
        let parser = |input: &str| -> Result<Vec<(u128, u128)>, ParsingError> {
            let mut lines = input.lines();
            let times = lines
                .next()
                .and_then(|line| line.strip_prefix("Time:"))
                .map(|line| line.trim())
                .ok_or(ParsingError)?
                .split_whitespace()
                .filter_map(|n| n.parse::<u128>().ok())
                .collect::<Vec<u128>>();
            let distances = lines
                .next()
                .and_then(|line| line.strip_prefix("Distance:"))
                .map(|line| line.trim())
                .ok_or(ParsingError)?
                .split_whitespace()
                .filter_map(|n| n.parse::<u128>().ok())
                .collect::<Vec<u128>>();

            Ok(times
                .iter()
                .enumerate()
                .filter_map(|(index, time)| distances.get(index).map(|distance| (*time, *distance)))
                .collect::<Vec<(u128, u128)>>())
        };

        let games = self.parse_input(parser).unwrap();
        let res = games
            .iter()
            .map(|g| ways_to_win(g.0, g.1))
            .reduce(|acc, x| acc * x)
            .unwrap();

        println!("Part 1: {}", res);
    }

    fn run_part_two(&self) {
        let parser = |input: &str| -> Result<(u128, u128), ParsingError> {
            let mut lines = input.lines();
            let time = lines
                .next()
                .and_then(|line| line.strip_prefix("Time:"))
                .map(|line| line.trim())
                .ok_or(ParsingError)?
                .split_whitespace()
                .collect::<String>()
                .parse::<u128>()
                .map_err(|_| ParsingError)?;
            let distance = lines
                .next()
                .and_then(|line| line.strip_prefix("Distance:"))
                .map(|line| line.trim())
                .ok_or(ParsingError)?
                .split_whitespace()
                .collect::<String>()
                .parse::<u128>()
                .map_err(|_| ParsingError)?;

            Ok((time, distance))
        };

        let game = self.parse_input(parser).unwrap();
        let res = ways_to_win(game.0, game.1);

        println!("Part 2: {}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(ways_to_win(7, 9), 4);
        assert_eq!(ways_to_win(15, 40), 8);
        assert_eq!(ways_to_win(30, 200), 9);
    }
}
