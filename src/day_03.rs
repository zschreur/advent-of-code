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
struct SchematicValue<T> {
    value: T,
    pos: Position,
    length: usize,
}

impl<T> SchematicValue<T> {
    fn is_adjacent<A>(&self, other: &SchematicValue<A>) -> bool {
        let adjacent_x = if self.pos.x >= other.pos.x {
            self.pos.x - other.pos.x <= other.length
        } else {
            other.pos.x - self.pos.x <= self.length
        };

        let adjacent_y = if self.pos.y >= other.pos.y {
            self.pos.y - other.pos.y <= 1
        } else {
            other.pos.y - self.pos.y <= 1
        };

        adjacent_x && adjacent_y
    }
}

type SchematicNumber = SchematicValue<usize>;
type SchematicSymbol = SchematicValue<char>;

impl SchematicNumber {
    fn create(value: usize, x: usize, y: usize, length: usize) -> Self {
        Self {
            value,
            pos: Position { x, y },
            length: length,
        }
    }
}

impl SchematicSymbol {
    fn create(value: char, x: usize, y: usize) -> Self {
        Self {
            value,
            pos: Position { x, y },
            length: 1,
        }
    }
}

#[derive(Debug)]
struct EngineSchematic {
    symbols: Vec<SchematicSymbol>,
    schematic_numbers: Vec<SchematicValue<usize>>,
}

fn parse_board(puzzle_input: &str) -> EngineSchematic {
    let board = puzzle_input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let board = Board::create(board);

    let mut symbols: Vec<SchematicSymbol> = vec![];
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
                        current_number = Some(SchematicNumber::create(d, x, y, 1));
                    }
                }
                _ => {
                    if let Some(s) = &current_number {
                        schematic_numbers.push(*s);
                    }
                    current_number = None;

                    if *c != '.' {
                        symbols.push(SchematicSymbol::create(*c, x, y))
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

        let res = engine_schematic
            .schematic_numbers
            .iter()
            .filter(|s| {
                engine_schematic
                    .symbols
                    .iter()
                    .any(|symbol| s.is_adjacent(symbol))
            })
            .fold(0, |acc, s| acc + s.value);

        println!("Part 1: {}", res);
    }

    fn run_part_two(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let schematic = parse_board(&"12..34\n*.123*\n...#45");
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

    #[test]
    fn test_is_adjacent() {
        let a = SchematicNumber::create(123, 5, 5, 3);

        let b = SchematicSymbol::create('*', 5, 4);
        assert!(a.is_adjacent(&b));
        assert!(b.is_adjacent(&a));

        let b = SchematicSymbol::create('*', 4, 5);
        assert!(a.is_adjacent(&b));
        assert!(b.is_adjacent(&a));

        let b = SchematicSymbol::create('*', 4, 4);
        assert!(a.is_adjacent(&b));
        assert!(b.is_adjacent(&a));

        let b = SchematicSymbol::create('*', 8, 6);
        assert!(a.is_adjacent(&b));
        assert!(b.is_adjacent(&a));
    }
}
