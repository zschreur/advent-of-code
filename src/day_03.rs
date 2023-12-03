pub struct Puzzle {
    puzzle_input: String,
}

impl Puzzle {
    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self {
            puzzle_input: input,
        })
    }
}

struct Board(Vec<Vec<char>>);

impl Board {
    fn create(board: Vec<Vec<char>>) -> Self {
        Self(board)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SchematicNumber {
    value: usize,
    pos: Position,
    length: usize,
}

#[derive(Debug)]
struct EngineSchematic {
    symbols: Vec<Position>,
    schematic_numbers: Vec<SchematicNumber>,
}

fn parse_board(puzzle_input: &str) -> EngineSchematic {
    let board = puzzle_input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let board = Board::create(board);

    let mut symbols: Vec<Position> = vec![];
    let mut schematic_numbers: Vec<SchematicNumber> = vec![];

    board.0.iter().enumerate().for_each(|(y, line)| {
        let mut current_number: Option<SchematicNumber> = None;
        line.iter()
            .enumerate()
            .for_each(|(x, c)| match c.to_string().parse::<usize>() {
                Ok(d) => {
                    if let Some(s) = &mut current_number {
                        s.length = s.length + 1;
                        s.value = s.value * 10 + d;
                    } else {
                        current_number = Some(SchematicNumber {
                            value: d,
                            pos: Position { x, y },
                            length: 1,
                        })
                    }
                }
                _ => {
                    if let Some(s) = &current_number {
                        schematic_numbers.push(*s);
                    }
                    current_number = None;

                    if *c != '.' {
                        symbols.push(Position { x, y })
                    }
                }
            });
        if let Some(s) = &current_number {
            schematic_numbers.push(*s);
        }
    });

    return EngineSchematic {
        symbols,
        schematic_numbers,
    };
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {
        let engine_schematic = parse_board(&self.puzzle_input);
        println!("Part 1: {}", "NOT IMPLEMENTED");
    }

    fn run_part_two(&self) {
        println!("Part 2: {}", "NOT IMPLEMENTED");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..\
                    ...*......\
                    ..35..633.\
                    ......#...\
                    617*......\
                    .....+.58.\
                    ..592.....\
                    ......755.\
                    ...$.*....\
                    .664.598..";

    #[test]
    fn test_parse() {
        let schematic = parse_board(&"12..34\n*.123.*\n...#/45");
        assert_eq!(schematic.schematic_numbers.len(), 4);
        assert_eq!(
            *schematic.schematic_numbers.get(0).unwrap(),
            SchematicNumber {
                value: 12,
                pos: Position { x: 0, y: 0 },
                length: 2
            }
        );
        assert_eq!(
            *schematic.schematic_numbers.get(1).unwrap(),
            SchematicNumber {
                value: 34,
                pos: Position { x: 4, y: 0 },
                length: 2
            }
        );
        assert_eq!(
            *schematic.schematic_numbers.get(2).unwrap(),
            SchematicNumber {
                value: 123,
                pos: Position { x: 2, y: 1 },
                length: 3
            }
        );
    }
}
