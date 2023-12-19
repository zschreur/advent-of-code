/*
 * The contraption appears to be a flat,
 * two-dimensional square grid containing
 * empty space (.), mirrors (/ and \), and splitters (| and -)
 *
 * The beam enters in the top-left corner from the left and heading to the right.
 * Then, its behavior depends on what it encounters as it moves:
 * If the beam encounters empty space (.), it continues in the same direction.
 * If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees
 *   depending on the angle of the mirror. For instance, a rightward-moving beam
 *   that encounters a / mirror would continue upward in the mirror's column,
 *   while a rightward-moving beam that encounters a \ mirror would continue
 *   downward from the mirror's column.
 * If the beam encounters the pointy end of a splitter (| or -),
 *   the beam passes through the splitter as if the splitter were empty space.
 *   For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
 * If the beam encounters the flat side of a splitter (| or -),
 *   the beam is split into two beams going in each of the two
 *   directions the splitter's pointy ends are pointing.
 *   For instance, a rightward-moving beam that encounters a | splitter would
 *   split into two beams: one that continues upward from
 *   the splitter's column and one that continues downward from the splitter's column.
 *
 * Beams do not interact with other beams; a tile can have many beams passing through
 * it at the same time. A tile is energized if that tile has at least one beam pass through it, reflect in it, or split in it.
 *
 * Count the number of tiles that become energized.
 */

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

struct Grid {
    tiles: Vec<Tile>,
    size: usize,
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
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '.' => Tile::Empty,
                '/' => Tile::FMirror,
                '\\' => Tile::BMirror,
                '|' => Tile::VSplitter,
                '-' => Tile::HSplitter,
                _ => panic!("Invalid character"),
            })
            .collect::<Vec<_>>();
        let size = input.lines().count();
        Self { tiles: grid, size }
    }

    fn _get(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.size || y >= self.size {
            return None;
        }
        Some(self.tiles[y * self.size + x])
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
        let _grid = Grid::new(&self.0);
    }

    fn run_part_two(&self) {}
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
        let grid = Grid::new(SAMPLE);
        println!("{:?}", &grid);
        assert_eq!(grid.size, 10);
    }
}
