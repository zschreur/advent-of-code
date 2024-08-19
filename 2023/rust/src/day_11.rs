#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct Image {
    galaxies: Vec<Position>,
    expanded_rows: Vec<u64>,
    expanded_columns: Vec<u64>,
}

fn parse_image(input: &str) -> Option<Image> {
    let size = input.lines().count();
    let mut expanded_rows = Vec::with_capacity(size);
    let mut columns_with_galaxies = vec![false; size];
    let mut count = 0;
    let galaxies =
        input
            .lines()
            .enumerate()
            .fold(Vec::<Position>::new(), |mut positions, (y, line)| {
                let row = line
                    .chars()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        '#' => {
                            columns_with_galaxies[x] = true;
                            Some(Position { x, y })
                        }
                        _ => None,
                    })
                    .collect::<Vec<Position>>();

                if row.len() == 0 {
                    count += 1;
                }
                expanded_rows.push(count);

                positions.extend(row);
                positions
            });

    count = 0;
    let mut expanded_columns = Vec::with_capacity(size);
    for x in columns_with_galaxies {
        if !x {
            count += 1;
        }
        expanded_columns.push(count);
    }

    Some(Image {
        expanded_rows,
        expanded_columns,
        galaxies,
    })
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

fn galaxy_distances(image: &Image, expansion_size: u64) -> u64 {
    let galaxies = &image.galaxies;
    let mut sum: u64 = 0;
    for (i, Position { x: x0, y: min_y }) in galaxies.iter().enumerate() {
        for Position { x: x1, y: max_y } in galaxies.iter().skip(i + 1) {
            let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };

            let expanded_column_count =
                image.expanded_columns[*max_x] - image.expanded_columns[*min_x];

            let expanded_row_count = image.expanded_rows[*max_y] - image.expanded_rows[*min_y];

            sum += (expansion_size - 1) * (expanded_row_count + expanded_column_count)
                + (max_x - min_x) as u64
                + (max_y - min_y) as u64;
        }
    }

    sum
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let image = parse_image(&self.0).expect("Issue parsing image");
        let total_distance = galaxy_distances(&image, 2);

        Ok(super::AOCResult::U64(total_distance))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let image = parse_image(&self.0).expect("Issue parsing image");
        let total_distance = galaxy_distances(&image, 1_000_000);

        Ok(super::AOCResult::U64(total_distance))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test() {
        let image = parse_image(&SAMPLE_INPUT).unwrap();
        assert_eq!(galaxy_distances(&image, 2), 374);
        assert_eq!(galaxy_distances(&image, 100), 8410);
    }
}
