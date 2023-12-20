use std::str::FromStr;

struct HandCalculator;

impl HandCalculator {
    fn get_bid(hand: u128) -> u128 {
        hand & 0x3FF
    }

    #[allow(unused)]
    fn get_rank(hand: u128) -> u128 {
        hand >> 30
    }

    #[allow(unused)]
    fn get_cards(hand: u128) -> u128 {
        (hand >> 10) & 0xFFFFF
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct JokerHand(u128);
impl JokerHand {
    fn parse_rank(s: &str) -> u128 {
        let mut cards = s
            .chars()
            .fold(std::collections::HashMap::new(), |mut acc, c| {
                if let Some(k) = acc.get_mut(&c) {
                    *k += 1u8;
                } else {
                    acc.insert(c, 1);
                }

                acc
            });

        let joker_count = *(cards.get(&'J').unwrap_or(&0));
        if let Some((card, count)) = cards
            .iter()
            .filter(|(card, _)| *card != &'J')
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .map(|(card, count)| (*card, *count))
        {
            if let Some(c) = cards.get_mut(&card) {
                *c = joker_count + count;
                cards.remove(&'J');
            }
        }

        let card_counts = cards.values().map(|v| *v).collect::<Vec<u8>>();
        match card_counts.len() {
            1 => 6,
            2 => {
                let first_card = card_counts[0];
                if first_card == 1 || first_card == 4 {
                    5
                } else {
                    4
                }
            }
            3 => {
                if card_counts.contains(&3) {
                    3
                } else {
                    2
                }
            }
            4 => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct BasicHand(u128);
impl BasicHand {
    fn parse_rank(s: &str) -> u128 {
        let cards = s
            .chars()
            .fold(std::collections::HashMap::new(), |mut acc, c| {
                if let Some(k) = acc.get_mut(&c) {
                    *k += 1u8;
                } else {
                    acc.insert(c, 1);
                }

                acc
            });

        let card_counts = cards.values().map(|v| *v).collect::<Vec<u8>>();
        match card_counts.len() {
            1 => 6,
            2 => {
                let first_card = card_counts[0];
                if first_card == 1 || first_card == 4 {
                    5
                } else {
                    4
                }
            }
            3 => {
                if card_counts.contains(&3) {
                    3
                } else {
                    2
                }
            }
            4 => 1,
            _ => 0,
        }
    }
}

struct ParseHandError;
impl FromStr for BasicHand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_at(5);
        let bid = bid.trim().parse::<u128>().map_err(|_| ParseHandError)?;

        let rank = BasicHand::parse_rank(&cards);
        let cards = cards
            .chars()
            .filter_map(|c| -> Option<u128> {
                match c {
                    'A' => Some(0xE),
                    'K' => Some(0xD),
                    'Q' => Some(0xC),
                    'J' => Some(0xB),
                    'T' => Some(0xA),
                    d if d.is_ascii_digit() => Some(d as u128 - '0' as u128),
                    _ => None,
                }
            })
            .reduce(|acc, d| (acc << 4) | d)
            .ok_or(ParseHandError)?;

        let hand = (((rank << 20) | cards) << 10) | bid;
        Ok(BasicHand(hand))
    }
}

impl FromStr for JokerHand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_at(5);
        let bid = bid.trim().parse::<u128>().map_err(|_| ParseHandError)?;

        let rank = JokerHand::parse_rank(&cards);
        let cards = cards
            .chars()
            .filter_map(|c| -> Option<u128> {
                match c {
                    'A' => Some(0xE),
                    'K' => Some(0xD),
                    'Q' => Some(0xC),
                    'J' => Some(0x0),
                    'T' => Some(0xA),
                    d if d.is_ascii_digit() => Some(d as u128 - '0' as u128),
                    _ => None,
                }
            })
            .reduce(|acc, d| (acc << 4) | d)
            .ok_or(ParseHandError)?;

        let hand = (((rank << 20) | cards) << 10) | bid;
        Ok(JokerHand(hand))
    }
}

#[derive(Debug)]
struct Game<T: FromStr>(Vec<T>);

#[derive(Debug)]
pub struct Puzzle(String);

impl Puzzle {
    fn new(input: &str) -> Self {
        Self(input.to_string())
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }

    fn parse_input<T: FromStr>(&self) -> Vec<T> {
        self.0
            .lines()
            .filter_map(|line| line.parse::<T>().ok())
            .collect::<Vec<T>>()
    }

    fn get_game<T: FromStr>(&self) -> Game<T> {
        Game(self.parse_input::<T>())
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let res = {
            let mut game = self.get_game::<BasicHand>();
            game.0.sort();
            game.0.iter().enumerate().fold(0u128, |acc, (index, h)| {
                ((index as u128 + 1) * HandCalculator::get_bid(h.0)) + acc
            })
        };

        Ok(super::AOCResult::ULong(res as u128))
    }
    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let res = {
            let mut game = self.get_game::<JokerHand>();
            game.0.sort();
            game.0.iter().enumerate().fold(0u128, |acc, (index, h)| {
                ((index as u128 + 1) * HandCalculator::get_bid(h.0)) + acc
            })
        };

        Ok(super::AOCResult::ULong(res as u128))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle::new(&SAMPLE_INPUT);
        let game = puzzle.get_game::<BasicHand>();
        let first_hand_bid = HandCalculator::get_bid(game.0.first().unwrap().0);
        let first_hand_rank = HandCalculator::get_rank(game.0.first().unwrap().0);
        let first_hand_cards = HandCalculator::get_cards(game.0.first().unwrap().0);
        assert_eq!(first_hand_bid, 765);
        assert_eq!(first_hand_rank, 1); // One pair
        assert_eq!(first_hand_cards, 0x32A3D); // One pair
        assert_eq!(puzzle.0.len(), 5);
    }

    #[test]
    fn test_parse_rank() {
        assert_eq!(BasicHand::parse_rank("AAAAA"), 6);
        assert_eq!(BasicHand::parse_rank("AAAAQ"), 5);
        assert_eq!(BasicHand::parse_rank("AAAQQ"), 4);
        assert_eq!(BasicHand::parse_rank("QQAAA"), 4);
        assert_eq!(BasicHand::parse_rank("AAAKQ"), 3);
        assert_eq!(BasicHand::parse_rank("KQAAA"), 3);
        assert_eq!(BasicHand::parse_rank("KAAAQ"), 3);
        assert_eq!(BasicHand::parse_rank("AKAQA"), 3);
        assert_eq!(BasicHand::parse_rank("QKKAA"), 2);
        assert_eq!(BasicHand::parse_rank("KKQAA"), 2);
        assert_eq!(BasicHand::parse_rank("KKAAQ"), 2);
        assert_eq!(BasicHand::parse_rank("KQKAA"), 2);
        assert_eq!(BasicHand::parse_rank("AAKQJ"), 1);
        assert_eq!(BasicHand::parse_rank("KAAQJ"), 1);
        assert_eq!(BasicHand::parse_rank("KQAAJ"), 1);
        assert_eq!(BasicHand::parse_rank("KQAJA"), 1);
        assert_eq!(BasicHand::parse_rank("KQJAA"), 1);
        assert_eq!(BasicHand::parse_rank("AKQJT"), 0);
    }
}
