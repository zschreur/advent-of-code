#[derive(Debug)]
struct ScratchCard {
    numbers: std::collections::HashSet<u32>,
    winning: std::collections::HashSet<u32>,
}

#[derive(Debug)]
pub struct Puzzle {
    scratch_cards: Vec<ScratchCard>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let scratch_cards = Self::parse_input(&input);
        Self { scratch_cards }
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }

    fn parse_input(input: &str) -> Vec<ScratchCard> {
        input
            .lines()
            .filter_map(|line| Self::parse_line(&line))
            .collect::<Vec<ScratchCard>>()
    }

    fn parse_line(line: &str) -> Option<ScratchCard> {
        line.find(':')
            .map(|index| line.split_at(index + 1).1)
            .and_then(|card_values| {
                let mid = card_values.find('|')?;
                Some(card_values.split_at(mid))
            })
            .map(|(numbers, winning)| {
                let parse_numbers = |s: &str| -> std::collections::HashSet<u32> {
                    s.split_whitespace()
                        .filter_map(|n| n.parse::<u32>().ok())
                        .collect::<std::collections::HashSet<u32>>()
                };
                let numbers = parse_numbers(numbers);
                let winning = parse_numbers(winning);

                ScratchCard { numbers, winning }
            })
    }

    fn calculate_points(&self) -> u32 {
        self.scratch_cards
            .iter()
            .map(|card| {
                let winning_number_count = card.winning.intersection(&card.numbers).count() as u32;
                match winning_number_count.checked_sub(1) {
                    Some(exponent) => u32::pow(2, exponent),
                    None => 0,
                }
            })
            .sum()
    }

    fn total_scratch_cards(&self) -> usize {
        let mut count: Vec<usize> = vec![1; self.scratch_cards.len()];
        for (index, card) in self.scratch_cards.iter().enumerate() {
            let winning_number_count = card.winning.intersection(&card.numbers).count();
            (0..winning_number_count).for_each(|j| {
                let current_card_count = match count.get(index) {
                    Some(v) => *v,
                    None => 0,
                };
                if let Some(elem) = count.get_mut(index + j + 1) {
                    *elem += current_card_count;
                }
            });
        }

        count.iter().sum()
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let result = self.calculate_points();

        Ok(super::AOCResult::U32(result))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let result = self.total_scratch_cards();

        Ok(super::AOCResult::USize(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_sample_part_one() {
        let puzzle = Puzzle::new(SAMPLE_INPUT);
        assert_eq!(puzzle.calculate_points(), 13);
    }

    #[test]
    fn test_sample_part_two() {
        let puzzle = Puzzle::new(SAMPLE_INPUT);
        assert_eq!(puzzle.total_scratch_cards(), 30);
    }
}
