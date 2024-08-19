use crate::grid::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    point: Point,
    count: usize,
    direction: Direction,
}

impl Node {
    fn new(point: Point, direction: Direction, count: usize) -> Self {
        Self {
            point,
            direction,
            count,
        }
    }

    fn as_index(&self) -> usize {
        let direction = match self.direction {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        };
        direction * 10 + self.count - 1
    }
}

fn djikstra(grid: &Grid<usize>, ultra: bool) -> Option<usize> {
    let mut visited_nodes = vec![vec![[false; 40]; grid.size()]; grid.size()];
    let mut unvisited_nodes = BTreeMap::<usize, BTreeSet<Node>>::new();
    unvisited_nodes.insert(
        *grid.get(Point(0, 1)).unwrap(),
        BTreeSet::from_iter(vec![Node::new(Point(0, 1), Direction::Down, 1)]),
    );
    unvisited_nodes.insert(
        *grid.get(Point(1, 0)).unwrap(),
        BTreeSet::from_iter(vec![Node::new(Point(1, 0), Direction::Right, 1)]),
    );

    loop {
        let (current_node, current_cost) = {
            let mut min_value = match unvisited_nodes.first_entry() {
                Some(min_value) => min_value,
                None => {
                    break;
                }
            };
            let cost = *min_value.key();
            let node = match min_value.get_mut().pop_last() {
                Some(node) => node,
                _ => {
                    break;
                }
            };

            if min_value.get().is_empty() {
                min_value.remove_entry();
            }

            (node, cost)
        };
        if current_node.point.0 == grid.size() - 1 && current_node.point.1 == grid.size() - 1 {
            return Some(current_cost);
        }

        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter(|direction| match (**direction, current_node.direction) {
            (Direction::Up, Direction::Down) => false,
            (Direction::Down, Direction::Up) => false,
            (Direction::Left, Direction::Right) => false,
            (Direction::Right, Direction::Left) => false,
            _ => true,
        })
        .filter_map(|direction| {
            if direction == &current_node.direction {
                if ultra {
                    if current_node.count < 10 {
                        Some((*direction, current_node.count + 1))
                    } else {
                        None
                    }
                } else if current_node.count >= 3 {
                    None
                } else {
                    Some((*direction, current_node.count + 1))
                }
            } else if ultra && current_node.count < 4 {
                None
            } else {
                Some((*direction, 1))
            }
        })
        .filter_map(|(direction, count)| {
            current_node
                .point
                .move_direction(direction)
                .and_then(|point| {
                    grid.get(point).map(|v| {
                        (
                            Node {
                                point,
                                direction,
                                count,
                            },
                            v + current_cost,
                        )
                    })
                })
        })
        .filter(|(node, _)| visited_nodes[node.point.0][node.point.1][node.as_index()] == false)
        .for_each(|(node, value)| {
            unvisited_nodes
                .entry(value)
                .or_insert_with(BTreeSet::new)
                .insert(node);
        });

        visited_nodes[current_node.point.0][current_node.point.1][current_node.as_index()] = true;
    }

    None
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
        let blocks = self
            .0
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Not a digit") as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let grid = Grid::<_>::new(blocks.len(), blocks);
        let res = djikstra(&grid, false)
            .ok_or("No path found")
            .map(|v| super::AOCResult::USize(v))?;

        Ok(res)
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let blocks = self
            .0
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Not a digit") as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let grid = Grid::<_>::new(blocks.len(), blocks);
        let res = djikstra(&grid, true)
            .ok_or("No path found")
            .map(|v| super::AOCResult::USize(v))?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test() {
        let blocks = SAMPLE_INPUT
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Not a digit") as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let grid = Grid::<_>::new(SAMPLE_INPUT.lines().count(), blocks);
        let res = djikstra(&grid, false);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 102);
    }
}
