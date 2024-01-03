use std::str::FromStr;

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
        unimplemented!("Part one not implemented")
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        unimplemented!("Part two not implemented")
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct ColorCode {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: u32,
    color: ColorCode,
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, rest) = s.split_at(1);
        let direction = match direction {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Unexpected direction"),
        }?;

        let rest = rest.trim();
        let mid = rest.trim().find(' ').ok_or("Missing space")?;
        let (distance, rest) = rest.split_at(mid);

        let distance = distance.parse::<u32>()?;

        let color = rest
            .strip_prefix(" (#")
            .and_then(|s| s.strip_suffix(")"))
            .ok_or("Issue parsing color")?;

        let (red, rest) = color.split_at(2);
        let (green, blue) = rest.split_at(2);
        dbg!(red, green, blue);

        let color = ColorCode {
            r: u8::from_str_radix(red, 16)?,
            g: u8::from_str_radix(green, 16)?,
            b: u8::from_str_radix(blue, 16)?,
        };

        Ok(Self {
            direction,
            distance,
            color,
        })
    }
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
        let instruction = "R 6 (#70c710)".parse::<Instruction>().unwrap();
        assert_eq!(instruction.direction, Direction::Right);
        assert_eq!(instruction.distance, 6);
        assert_eq!(
            instruction.color,
            ColorCode {
                r: 0x70,
                g: 0xc7,
                b: 0x10
            }
        );
    }

    #[test]
    fn test_sample_part_one() {
    }
}
