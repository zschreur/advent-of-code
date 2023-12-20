use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    FMirror,
    BMirror,
    VSplitter,
    HSplitter,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::FMirror => '/',
            Tile::BMirror => '\\',
            Tile::VSplitter => '|',
            Tile::HSplitter => '-',
        };
        write!(f, "{}", c)
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct IndexOutOfBoundsError;
impl Position {
    fn move_direction(&self, dir: Direction) -> Result<Self, IndexOutOfBoundsError> {
        let pos = match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y.checked_sub(1).ok_or(IndexOutOfBoundsError)?,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x.checked_sub(1).ok_or(IndexOutOfBoundsError)?,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        };

        Ok(pos)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum CollisionResult {
    Continue,
    Turn(Direction),
    Split(Direction, Direction),
}

impl Tile {
    fn collision(&self, dir: Direction) -> CollisionResult {
        match self {
            Tile::Empty => CollisionResult::Continue,
            Tile::FMirror => match dir {
                Direction::Up => CollisionResult::Turn(Direction::Right),
                Direction::Down => CollisionResult::Turn(Direction::Left),
                Direction::Left => CollisionResult::Turn(Direction::Down),
                Direction::Right => CollisionResult::Turn(Direction::Up),
            },
            Tile::BMirror => match dir {
                Direction::Up => CollisionResult::Turn(Direction::Left),
                Direction::Down => CollisionResult::Turn(Direction::Right),
                Direction::Left => CollisionResult::Turn(Direction::Up),
                Direction::Right => CollisionResult::Turn(Direction::Down),
            },
            Tile::VSplitter => match dir {
                Direction::Left | Direction::Right => {
                    CollisionResult::Split(Direction::Up, Direction::Down)
                }
                Direction::Down | Direction::Up => CollisionResult::Continue,
            },
            Tile::HSplitter => match dir {
                Direction::Up | Direction::Down => {
                    CollisionResult::Split(Direction::Left, Direction::Right)
                }
                Direction::Left | Direction::Right => CollisionResult::Continue,
            },
        }
    }
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Tile>,
    size: usize,
    beams: Vec<(Position, Direction)>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tiles.iter().enumerate().try_for_each(|(i, c)| {
            if i % self.size == 0 && i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", c)
        })
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Grid {
    fn new(tiles: Vec<Tile>, size: usize) -> Self {
        Self {
            tiles,
            size,
            beams: vec![],
        }
    }

    fn from_input(input: &str) -> Result<Self, ()> {
        let grid = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '.' => Ok(Tile::Empty),
                '/' => Ok(Tile::FMirror),
                '\\' => Ok(Tile::BMirror),
                '|' => Ok(Tile::VSplitter),
                '-' => Ok(Tile::HSplitter),
                _ => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()?;

        let size = input.lines().count();
        Ok(Grid::new(grid, size))
    }

    fn get(&self, position: Position) -> Option<Tile> {
        if position.x >= self.size || position.y >= self.size {
            return None;
        }
        Some(self.tiles[position.y * self.size + position.x])
    }

    fn add_beam(&mut self, position: Position, direction: Direction) {
        self.beams.push((position, direction));
    }

    fn tick(&mut self) {
        let mut new_beams = vec![];
        self.beams = self
            .beams
            .iter()
            .filter_map(|(pos, dir)| {
                let tile = self.get(*pos).unwrap();
                match tile.collision(*dir) {
                    CollisionResult::Continue => match pos.move_direction(*dir).ok()? {
                        Position { x, y } if x < self.size && y < self.size => {
                            Some((Position { x, y }, *dir))
                        }
                        _ => None,
                    },
                    CollisionResult::Turn(new_dir) => match pos.move_direction(new_dir).ok()? {
                        Position { x, y } if x < self.size && y < self.size => {
                            Some((Position { x, y }, new_dir))
                        }
                        _ => None,
                    },
                    CollisionResult::Split(dir1, dir2) => {
                        let new_beam = match pos.move_direction(dir1) {
                            Ok(Position { x, y }) if x < self.size && y < self.size => {
                                Some((Position { x, y }, dir1))
                            }
                            _ => None,
                        };

                        match new_beam {
                            Some(b) => new_beams.push(b),
                            _ => (),
                        };

                        match pos.move_direction(dir2).ok()? {
                            Position { x, y } if x < self.size && y < self.size => {
                                Some((Position { x, y }, dir2))
                            }
                            _ => None,
                        }
                    }
                }
            })
            .collect();

        self.beams.extend(new_beams);
    }

    fn run_simulation(&mut self) -> u128 {
        let mut seen = self.beams.iter().map(|b| *b).collect::<HashSet<_>>();
        loop {
            if self.beams.is_empty() {
                break;
            }

            self.beams.iter().for_each(|b| {
                seen.insert(*b);
            });

            self.tick();
            self.beams = self
                .beams
                .iter()
                .filter(|b| !seen.contains(b))
                .map(|b| *b)
                .collect();
        }

        seen.iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>()
            .len() as u128
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
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let mut grid = Grid::from_input(&self.0).map_err(|_| "Invalid input")?;
        grid.add_beam(Position { x: 0, y: 0 }, Direction::Right);

        Ok(super::AOCResult::ULong(grid.run_simulation()))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let grid = Grid::from_input(&self.0).map_err(|_| "Invalid input")?;
        let res = (0..grid.size)
            .filter_map(|i| {
                let b1 = (Position { x: i, y: 0 }, Direction::Down);
                let b2 = (
                    Position {
                        x: i,
                        y: grid.size - 1,
                    },
                    Direction::Up,
                );
                let b3 = (Position { x: 0, y: i }, Direction::Right);
                let b4 = (
                    Position {
                        x: grid.size - 1,
                        y: i,
                    },
                    Direction::Left,
                );
                [b1, b2, b3, b4]
                    .iter()
                    .filter_map(|b| {
                        let mut grid = grid.clone();
                        grid.add_beam(b.0, b.1);
                        Some(grid.run_simulation())
                    })
                    .max()
            })
            .max()
            .ok_or("No solution")?;
        Ok(super::AOCResult::ULong(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part_one_sample() {
        let mut g = Grid::from_input(SAMPLE).unwrap();
        g.add_beam(Position { x: 0, y: 0 }, Direction::Right);
        assert_eq!(g.beams.len(), 1);
        assert_eq!(g.beams[0].0, Position { x: 0, y: 0 });
        assert_eq!(g.beams[0].1, Direction::Right);
        g.tick();
        assert_eq!(g.beams.len(), 1);
        assert_eq!(g.beams[0].0, Position { x: 1, y: 0 });
        assert_eq!(g.beams[0].1, Direction::Right);
        g.tick();
        assert_eq!(g.beams.len(), 1);
        assert_eq!(g.beams[0].0, Position { x: 1, y: 1 });
        assert_eq!(g.beams[0].1, Direction::Down);

        let mut g = Grid::from_input(SAMPLE).unwrap();
        g.add_beam(Position { x: 0, y: 0 }, Direction::Right);
        assert_eq!(g.run_simulation(), 46);
    }
}
