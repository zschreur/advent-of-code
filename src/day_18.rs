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
        let build_instructions = self
            .0
            .lines()
            .map(|s| Instruction::part_one_parse(s).unwrap())
            .collect::<Vec<_>>();

        let res = cubic_meters(&build_instructions);

        Ok(super::AOCResult::ULong(res as u128))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let build_instructions = self
            .0
            .lines()
            .map(|s| Instruction::part_two_parse(s).unwrap())
            .collect::<Vec<_>>();

        let res = cubic_meters(&build_instructions);

        Ok(super::AOCResult::ULong(res as u128))
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

#[derive(Debug)]
struct ParseInstructionError;
impl Instruction {
    fn part_two_parse(s: &str) -> Result<Self, ParseInstructionError> {
        let (distance, direction) = s
            .find('#')
            .and_then(|i| s.get(i + 1..i + 7))
            .map(|s| s.split_at(5))
            .expect("No color code");

        let distance = i32::from_str_radix(distance, 16).map_err(|_| ParseInstructionError)?;
        let direction = match direction {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => return Err(ParseInstructionError),
        };

        Ok(Self {
            direction,
            distance,
        })
    }

    fn part_one_parse(s: &str) -> Result<Self, ParseInstructionError> {
        let (direction, rest) = s.split_at(1);
        let direction = match direction {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseInstructionError),
        }?;

        let rest = rest.trim();
        let mid = rest.trim().find(' ').ok_or(ParseInstructionError)?;
        let (distance, _) = rest.split_at(mid);

        let distance = distance.parse::<i32>().map_err(|_| ParseInstructionError)?;

        Ok(Self {
            direction,
            distance,
        })
    }
}

fn cubic_meters(instructions: &[Instruction]) -> u128 {
    let vertices = instructions
        .iter()
        .fold(
            (
                Vec::<(i32, i32)>::with_capacity(instructions.len() - 1),
                (0, 0),
            ),
            |(mut v, p),
             Instruction {
                 direction,
                 distance,
             }| {
                v.push(p);
                match direction {
                    Direction::Up => (v, (p.0, p.1 - distance)),
                    Direction::Down => (v, (p.0, p.1 + distance)),
                    Direction::Left => (v, (p.0 - distance, p.1)),
                    Direction::Right => (v, (p.0 + distance, p.1)),
                }
            },
        )
        .0;

    let sum = vertices.windows(2).fold(0i128, |acc, w| {
        let x1 = w[0].0;
        let x2 = w[1].0;
        let y1 = w[0].1;
        let y2 = w[1].1;
        acc + ((x2 + x1) as i128 * (y2 - y1) as i128)
    });

    let area = sum.abs() as u128 >> 1;
    let boundary_points = instructions
        .iter()
        .fold(0, |acc, i| acc + i.distance as u128);

    area + (boundary_points >> 1) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_parse_instruction() {
        let instruction = Instruction::part_one_parse("R 6 (#70c710)").unwrap();
        assert_eq!(instruction.direction, Direction::Right);
        assert_eq!(instruction.distance, 6);
    }

    #[test]
    fn test_sample_part_one() {
        let build_instructions = SAMPLE_INPUT
            .lines()
            .map(|s| Instruction::part_one_parse(s).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(cubic_meters(&build_instructions), 62);
    }

    #[test]
    fn test_sample_part_two() {
        let build_instructions = SAMPLE_INPUT
            .lines()
            .map(|s| Instruction::part_two_parse(s).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(cubic_meters(&build_instructions), 952408144115);
    }
}
