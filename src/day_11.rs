use std::collections::BTreeSet;

#[derive(PartialEq, Clone, Copy, Debug)]
enum ImageData {
    Galaxy,
    Empty,
}

struct Position {
    x: usize,
    y: usize,
}

use ImageData::*;

struct Image {
    image_data: Vec<Vec<ImageData>>,
    expanded_rows: BTreeSet<u128>,
    expanded_columns: BTreeSet<u128>,
}

fn parse_image(input: &str) -> Option<Image> {
    let mut expanded_rows = BTreeSet::new();
    let mut expanded_columns = BTreeSet::new();
    let image_data = input.lines().enumerate().fold(
        Vec::<Vec<ImageData>>::new(),
        |mut image, (row_index, line)| {
            let row = line
                .chars()
                .map(|c| match c {
                    '#' => Galaxy,
                    _ => Empty,
                })
                .collect::<Vec<ImageData>>();

            if !row.contains(&Galaxy) {
                expanded_rows.insert(row_index as u128);
            }
            image.push(row);

            image
        },
    );

    for x in 0..image_data.first()?.len() {
        if !image_data.iter().any(|row| row[x] == Galaxy) {
            expanded_columns.insert(x as u128);
        }
    }

    Some(Image {
        image_data,
        expanded_rows,
        expanded_columns,
    })
}

fn find_galaxies(image: &Vec<Vec<ImageData>>) -> Vec<Position> {
    image
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, row)| {
            acc.extend(row.iter().enumerate().filter_map(|(x, d)| match d {
                Galaxy => Some(Position { x, y }),
                _ => None,
            }));
            acc
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

fn galaxy_distances(image: &Image, expansion_size: u128) -> u128 {
    let galaxies = find_galaxies(&image.image_data);
    let mut sum: u128 = 0;
    for (i, Position { x: x0, y: y0 }) in galaxies.iter().enumerate() {
        for Position { x: x1, y: y1 } in galaxies.iter().skip(i) {
            let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
            let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
            let (min_x, min_y, max_x, max_y) = (
                *min_x as u128,
                *min_y as u128,
                *max_x as u128,
                *max_y as u128,
            );

            let expanded_column_count = image.expanded_columns.range(min_x..max_x).count() as u128;
            let expanded_row_count = image.expanded_rows.range(min_y..max_y).count() as u128;

            let dx = max_x - min_x + expanded_column_count * (expansion_size - 1);
            let dy = max_y - min_y + expanded_row_count * (expansion_size - 1);

            let path_length = (dx.min(dy) * 2) + dx.max(dy) - dx.min(dy);
            sum += path_length;
        }
    }

    sum
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let image = parse_image(&self.0).expect("Issue parsing image");
        let total_distance = galaxy_distances(&image, 2);

        Ok(super::AOCResult::ULong(total_distance))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let image = parse_image(&self.0).expect("Issue parsing image");
        let total_distance = galaxy_distances(&image, 1_000_000);

        Ok(super::AOCResult::ULong(total_distance))
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

    /*
    ....1........
    .........2...
    3............
    .............
    .............
    ........4....
    .5...........
    .##.........6
    ..##.........
    ...##........
    ....##...7...
    8....9.......
    */

    #[test]
    fn test() {
        let image = parse_image(&SAMPLE_INPUT).unwrap();
        assert_eq!(image.image_data.len(), 12);
        assert_eq!(image.image_data[0].len(), 13);
    }
}
