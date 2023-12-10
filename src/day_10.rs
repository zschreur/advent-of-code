use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Map = Vec<Vec<Option<Pipe>>>;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Position {
    x: usize,
    y: usize,
}

fn get_initial_state(map: &Map, starting_position: &Position) -> Pipe {
    let north = map
        .get(starting_position.y - 1)
        .and_then(|row| row.get(starting_position.x))
        .unwrap_or(&None);
    let north = match north {
        Some(Pipe::SouthEast) | Some(Pipe::SouthWest) | Some(Pipe::NorthSouth) => true,
        _ => false,
    };
    let south = map
        .get(starting_position.y + 1)
        .and_then(|row| row.get(starting_position.x))
        .unwrap_or(&None);
    let south = match south {
        Some(Pipe::NorthSouth) | Some(Pipe::NorthEast) | Some(Pipe::NorthWest) => true,
        _ => false,
    };
    let east = map
        .get(starting_position.y)
        .and_then(|row| row.get(starting_position.x + 1))
        .unwrap_or(&None);
    let east = match east {
        Some(Pipe::NorthWest) | Some(Pipe::SouthWest) | Some(Pipe::EastWest) => true,
        _ => false,
    };
    let west = map
        .get(starting_position.y)
        .and_then(|row| row.get(starting_position.x - 1))
        .unwrap_or(&None);
    let west = match west {
        Some(Pipe::SouthEast) | Some(Pipe::NorthEast) | Some(Pipe::EastWest) => true,
        _ => false,
    };

    match (north, south, east, west) {
        (true, true, _, _) => Pipe::NorthSouth,
        (true, _, true, _) => Pipe::NorthEast,
        (true, _, _, true) => Pipe::NorthWest,
        (_, true, true, _) => Pipe::SouthEast,
        (_, true, _, true) => Pipe::SouthWest,
        (_, _, true, true) => Pipe::EastWest,
        _ => panic!("Starting pipe is None"),
    }
}

struct Diagram {
    map: Map,
    starting_position: Position,
}

impl Diagram {
    fn new(mut map: Map, starting_position: Position) -> Self {
        let starting_pipe_type = get_initial_state(&map, &starting_position);
        let s = map
            .get_mut(starting_position.y)
            .and_then(|row| row.get_mut(starting_position.x))
            .unwrap();
        *s = Some(starting_pipe_type);

        Self {
            map,
            starting_position,
        }
    }
}

fn parse_diagram(input: &str) -> Diagram {
    let mut starting_position = None;
    let m = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '|' => Some(Pipe::NorthSouth),
                    '-' => Some(Pipe::EastWest),
                    'L' => Some(Pipe::NorthEast),
                    'J' => Some(Pipe::NorthWest),
                    '7' => Some(Pipe::SouthWest),
                    'F' => Some(Pipe::SouthEast),
                    'S' => {
                        starting_position = Some(Position { x, y });
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<Option<Pipe>>>()
        })
        .collect::<Map>();

    Diagram::new(
        m,
        starting_position.expect("Could not find starting position"),
    )
}

struct PipeNavigator<'a> {
    diagram: &'a Diagram,
    heading: Direction,
    position: Position,
    current_pipe: Pipe,
    visited: HashSet<Position>,
}

impl<'a> PipeNavigator<'a> {
    fn new(diagram: &'a Diagram) -> Self {
        let starting_pipe = diagram
            .map
            .get(diagram.starting_position.y)
            .unwrap()
            .get(diagram.starting_position.x)
            .unwrap()
            .unwrap();
        let heading = match starting_pipe {
            Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest => Direction::South,
            Pipe::SouthWest | Pipe::SouthEast => Direction::North,
            Pipe::EastWest => Direction::East,
        };
        let mut visited = HashSet::new();
        visited.insert(diagram.starting_position);

        Self {
            diagram,
            heading,
            visited,
            position: diagram.starting_position,
            current_pipe: starting_pipe,
        }
    }

    fn step(&mut self) {
        let (next_position, next_heading) = match (self.current_pipe, self.heading) {
            (Pipe::NorthSouth, Direction::North)
            | (Pipe::NorthWest, Direction::East)
            | (Pipe::NorthEast, Direction::West) => (
                Position {
                    x: self.position.x,
                    y: self.position.y - 1,
                },
                Direction::North,
            ),
            (Pipe::NorthSouth, Direction::South)
            | (Pipe::SouthWest, Direction::East)
            | (Pipe::SouthEast, Direction::West) => (
                Position {
                    x: self.position.x,
                    y: self.position.y + 1,
                },
                Direction::South,
            ),
            (Pipe::EastWest, Direction::West)
            | (Pipe::NorthWest, Direction::South)
            | (Pipe::SouthWest, Direction::North) => (
                Position {
                    x: self.position.x - 1,
                    y: self.position.y,
                },
                Direction::West,
            ),
            (Pipe::EastWest, Direction::East)
            | (Pipe::NorthEast, Direction::South)
            | (Pipe::SouthEast, Direction::North) => (
                Position {
                    x: self.position.x + 1,
                    y: self.position.y,
                },
                Direction::East,
            ),
            _ => panic!("Unable to find next position: {:?}", (&self.heading, &self.position, &self.current_pipe)),
        };

        self.current_pipe = match self
            .diagram
            .map
            .get(next_position.y)
            .and_then(|row| row.get(next_position.x))
        {
            Some(Some(pipe)) => *pipe,
            _ => panic!("Could not get next pipe"),
        };
        self.position = next_position;
        self.heading = next_heading;
        self.visited.insert(self.position);
    }

    fn is_at_starting_position(&self) -> bool {
        self.position == self.diagram.starting_position
    }
}

fn find_loop_points(diagram: &Diagram) -> HashSet<Position> {
    let mut pipe_navigator = PipeNavigator::new(&diagram);
    loop {
        pipe_navigator.step();
        if pipe_navigator.is_at_starting_position() {
            break;
        }
    }

    pipe_navigator.visited
}

enum State {
    Inside,
    Outside,
}

fn find_points_on_row(
    row: &Vec<Option<Pipe>>,
    loop_points: &HashSet<Position>,
    y: usize,
) -> HashSet<Position> {
    let mut collision: Option<Pipe> = None;
    let mut state = State::Outside;
    let mut points = HashSet::new();

    for (x, pipe) in row.iter().enumerate() {
        if loop_points.contains(&Position { x, y }) {
            let pipe = pipe.expect("Unexpected None loop point");
            match pipe {
                Pipe::NorthSouth => {
                    state = match state {
                        State::Inside => State::Outside,
                        State::Outside => State::Inside,
                    };
                    collision = None;
                }
                Pipe::NorthEast | Pipe::SouthEast => {
                    collision = Some(pipe);
                }
                Pipe::NorthWest if { collision == Some(Pipe::SouthEast) } => {
                    state = match state {
                        State::Inside => State::Outside,
                        State::Outside => State::Inside,
                    };
                    collision = None;
                }
                Pipe::SouthWest if { collision == Some(Pipe::NorthEast) } => {
                    state = match state {
                        State::Inside => State::Outside,
                        State::Outside => State::Inside,
                    };
                    collision = None;
                }
                Pipe::NorthWest | Pipe::SouthWest => {
                    if collision.is_none() {
                        panic!("Unexpected {:?} when collision is None", pipe)
                    }
                    collision = None;
                }
                Pipe::EastWest => {
                    if collision.is_none() {
                        panic!("Unexpected {:?} when collision is {:?}", pipe, collision);
                    }
                }
            }
        } else {
            match state {
                State::Inside => {
                    points.insert(Position { x, y });
                }
                _ => (),
            }
        }
    }

    points
}

struct LoopScanner<'a> {
    loop_points: &'a HashSet<Position>,
    map: &'a Map,
}

impl<'a> LoopScanner<'a> {
    fn new(loop_points: &'a HashSet<Position>, map: &'a Map) -> Self {
        Self { loop_points, map }
    }

    fn find_enclosed_points(&self) -> HashSet<Position> {
        let mut points = HashSet::new();
        for (y, row) in self.map.iter().enumerate() {
            let points_on_row = find_points_on_row(&row, &self.loop_points, y);
            points.extend(points_on_row.iter());
        }

        points
    }
}

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
    fn run_part_one(&self) {
        let diagram = parse_diagram(&self.0);
        let loop_points = find_loop_points(&diagram);

        let length = loop_points.len();

        println!("Part 1: {}", length / 2);
    }

    fn run_part_two(&self) {
        let diagram = parse_diagram(&self.0);
        let loop_points = find_loop_points(&diagram);
        let loop_scanner = LoopScanner::new(&loop_points, &diagram.map);
        let enclosed_points = loop_scanner.find_enclosed_points();
        let res = enclosed_points.len();

        println!("Part 2: {}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn test_parse_sample_input() {
        let map = parse_diagram(&SAMPLE_INPUT);
        assert_eq!(map.map.len(), 5);
        assert_eq!(map.map[0].len(), 5);
    }
}
