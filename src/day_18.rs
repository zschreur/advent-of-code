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
        let build_instructions = self
            .0
            .lines()
            .map(|s| s.parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();

        let res = cubic_meters(&build_instructions);

        Ok(super::AOCResult::ULong(res as u128))
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct ColorCode {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorCode {
    fn ansi_escape_code(&self) -> String {
        "\x1b[38;2;".to_string()
            + &self.r.to_string()
            + ";"
            + &self.g.to_string()
            + ";"
            + &self.b.to_string()
            + &"m".to_string()
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: usize,
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

        let distance = distance.parse::<usize>()?;

        let color = rest
            .strip_prefix(" (#")
            .and_then(|s| s.strip_suffix(")"))
            .ok_or("Issue parsing color")?;

        let (red, rest) = color.split_at(2);
        let (green, blue) = rest.split_at(2);

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

fn create_plot(instructions: &[Instruction]) -> Vec<Vec<Option<ColorCode>>> {
    let mut plot: Vec<Vec<Option<ColorCode>>> = vec![vec![None]];
    let mut pos = (0, 0);
    instructions.iter().for_each(|i| match i.direction {
        Direction::Up => {
            let len = pos.0 + 1;
            if i.distance > pos.1 {
                let mut v = vec![vec![]; i.distance - pos.1];
                v.extend_from_slice(&plot);
                plot = v;
                pos.1 = i.distance;
            }

            (0..i.distance).for_each(|d| {
                let row = &mut plot[pos.1 - d - 1];
                if len > row.len() {
                    row.resize_with(len, Default::default);
                }
                row[pos.0] = Some(i.color);
            });
            pos.1 -= i.distance;
        }
        Direction::Down => {
            let len = pos.0 + 1;
            (0..i.distance).for_each(|d| match plot.get_mut(pos.1 + 1 + d) {
                None => {
                    let mut row = vec![None; pos.0];
                    row.resize_with(len, Default::default);
                    row[pos.0] = Some(i.color);
                    plot.push(row);
                }
                Some(row) => {
                    if len > row.len() {
                        row.resize_with(len, Default::default);
                    }
                    row[pos.0] = Some(i.color);
                }
            });

            pos.1 += i.distance;
        }
        Direction::Left => {
            if i.distance > pos.0 {
                plot.iter_mut().for_each(|row| {
                    let mut v = vec![None; i.distance - pos.0];
                    v.extend_from_slice(row);
                    *row = v;
                });
                pos.0 = i.distance;
            }

            let row = &mut plot[pos.1];
            (0..i.distance).for_each(|d| {
                row[pos.0 - 1 - d] = Some(i.color);
            });
            pos.0 -= i.distance;
        }
        Direction::Right => {
            let len = pos.0 + i.distance + 1;
            let row = match plot.get_mut(pos.1) {
                None => {
                    plot.push(vec![None; len]);
                    &mut plot[pos.1]
                }
                Some(row) => {
                    if len > row.len() {
                        row.resize_with(len, Default::default);
                    }
                    row
                }
            };
            (0..i.distance).for_each(|d| {
                row[pos.0 + 1 + d] = Some(i.color);
            });
            pos.0 += i.distance;
        }
    });

    plot
}

type Plot = Vec<Vec<Option<ColorCode>>>;

fn fill(plot: &mut Plot, pos: (usize, usize), color: ColorCode) {
    let mut point = plot.get_mut(pos.1).and_then(|row| row.get_mut(pos.0));

    match point {
        Some(p) if p.is_none() => {
            *p = Some(color);
            fill(plot, (pos.0, pos.1 - 1), color);
            fill(plot, (pos.0, pos.1 + 1), color);
            fill(plot, (pos.0 - 1, pos.1), color);
            fill(plot, (pos.0 + 1, pos.1), color);
        }
        _ => (),
    };
}

fn fill_plot(plot: &mut Plot) {
    let fill_color = ColorCode {
        r: 0xff,
        g: 0xff,
        b: 0xff,
    };

    let start = plot
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, v)| match v {
                Some(_) => match (x.checked_sub(1).and_then(|x| row.get(x)), row.get(x + 1)) {
                    (Some(None), Some(None)) => Some((x + 1, y)),
                    (None, Some(None)) => Some((x + 1, y)),
                    _ => None,
                },
                None => None,
            })
        })
        .expect("No starting position");

    fill(plot, start, fill_color);
}

fn cubic_meters(instructions: &[Instruction]) -> usize {
    let mut plot = create_plot(&instructions);

    fill_plot(&mut plot);

    plot.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|v| v.is_some()).count()
    })
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

    const REST: &str = "";

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
        let build_instructions = SAMPLE_INPUT
            .lines()
            .map(|s| s.parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(cubic_meters(&build_instructions), 62);
    }
}
