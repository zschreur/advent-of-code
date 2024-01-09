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
    expanded_rows: Vec<u128>,
    expanded_columns: Vec<u128>,
}

fn parse_image(input: &str) -> Option<Image> {
    let capacity = input.lines().count();
    let mut expanded_rows = Vec::with_capacity(capacity);
    let mut expanded_columns = Vec::with_capacity(capacity);
    let mut count = 0;
    let image_data = input
        .lines()
        .fold(Vec::<Vec<ImageData>>::new(), |mut image, line| {
            let row = line
                .chars()
                .map(|c| match c {
                    '#' => Galaxy,
                    _ => Empty,
                })
                .collect::<Vec<ImageData>>();

            if !row.contains(&Galaxy) {
                count += 1;
            }
            expanded_rows.push(count);

            image.push(row);

            image
        });

    for x in 0..image_data.first()?.len() {
        if !image_data.iter().any(|row| row[x] == Galaxy) {
            count += 1;
        }
        expanded_columns.push(count);
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
    for (i, Position { x: x0, y: min_y }) in galaxies.iter().enumerate() {
        for Position { x: x1, y: max_y } in galaxies.iter().skip(i + 1) {
            let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };

            let expanded_column_count =
                image.expanded_columns[*max_x] - image.expanded_columns[*min_x];

            let expanded_row_count = image.expanded_rows[*max_y] - image.expanded_rows[*min_y];

            sum += (expansion_size - 1) * (expanded_row_count + expanded_column_count)
                + (max_x - min_x) as u128
                + (max_y - min_y) as u128;
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
    3322211100
    ...#...... 2
    .......#.. 2
    #......... 2
    .......... 1
    ......#... 1
    .#........ 1
    .........# 1
    .......... 0
    .......#.. 0
    #...#..... 0
    */

    #[test]
    fn test() {
        let image = parse_image(&SAMPLE_INPUT).unwrap();
        assert_eq!(image.image_data.len(), 10);
        assert_eq!(image.image_data[0].len(), 10);
        galaxy_distances(&image, 2);
    }
}
