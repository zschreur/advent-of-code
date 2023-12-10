enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    StartingPosition,
}

struct Maze {
    map: Vec<Vec<Option<Pipe>>>,
    starting_position: (usize, usize),
}

impl Maze {
    fn new(map: Vec<Vec<Option<Pipe>>>) -> Self {
        let starting_position = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                match row.iter().enumerate().find(|(_, node)| match node {
                    Some(Pipe::StartingPosition) => true,
                    _ => false,
                }) {
                    Some((x, _)) => Some((x, y)),
                    _ => None,
                }
            })
            .expect("No starting position in map");
        Self {
            map,
            starting_position,
        }
    }
}

fn parse_maze(input: &str) -> Maze {
    let m = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Some(Pipe::NorthSouth),
                    '-' => Some(Pipe::EastWest),
                    'L' => Some(Pipe::NorthEast),
                    'J' => Some(Pipe::NorthWest),
                    '7' => Some(Pipe::SouthWest),
                    'F' => Some(Pipe::SouthEast),
                    'S' => Some(Pipe::StartingPosition),
                    _ => None,
                })
                .collect::<Vec<Option<Pipe>>>()
        })
        .collect::<Vec<Vec<Option<Pipe>>>>();

    Maze::new(m)
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
        let maze = parse_maze(&self.0);
    }

    fn run_part_two(&self) {}
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
        let maze = parse_maze(&SAMPLE_INPUT);
        assert_eq!(maze.map.len(), 5);
        assert_eq!(maze.map[0].len(), 5);
        assert_eq!(maze.starting_position, (1, 1));
    }
}
