#[derive(Debug, PartialEq)]
struct Set {
    red: usize,
    blue: usize,
    green: usize,
}

struct Game {
    id: usize,
    sets: Vec<Set>,
}

fn parse_sets(game: &str) -> Vec<Set> {
    game.split(";")
        .map(|set| {
            set.split(",")
                .map(|color| {
                    let color = color.trim();
                    let mid = color.find(" ").unwrap();
                    let (count, color) = color.split_at(mid);
                    let count = count.parse::<usize>().unwrap();
                    (count, color.trim())
                })
                .fold(
                    Set {
                        red: 0,
                        blue: 0,
                        green: 0,
                    },
                    |acc, (count, color)| match color {
                        "red" => Set {
                            red: acc.red + count,
                            blue: acc.blue,
                            green: acc.green,
                        },
                        "blue" => Set {
                            red: acc.red,
                            blue: acc.blue + count,
                            green: acc.green,
                        },
                        "green" => Set {
                            red: acc.red,
                            blue: acc.blue,
                            green: acc.green + count,
                        },
                        _ => acc,
                    },
                )
        })
        .collect()
}

fn parse_game(game: &str) -> Option<Game> {
    let game = game.strip_prefix("Game ")?;
    let semicolon_position = game.find(":")?;
    let (game_id, game) = game.split_at(semicolon_position);
    let game = game.strip_prefix(":")?;
    let game_id = match game_id.parse::<usize>() {
        Ok(id) => Some(id),
        Err(_) => None,
    }?;

    Some(Game {
        id: game_id,
        sets: parse_sets(game),
    })
}

fn is_game_possible(game: &Game, constraint: &Set) -> bool {
    let s = game.sets.iter().fold(
        Set {
            blue: 0,
            red: 0,
            green: 0,
        },
        |acc, set| Set {
            blue: acc.blue.max(set.blue),
            green: acc.green.max(set.green),
            red: acc.red.max(set.red),
        },
    );

    s.blue <= constraint.blue && s.red <= constraint.red && s.green <= constraint.green
}

fn fewest_possible_cubes(game: &Game) -> Set {
    game.sets.iter().fold(
        Set {
            blue: 0,
            red: 0,
            green: 0,
        },
        |acc, set| Set {
            blue: acc.blue.max(set.blue),
            green: acc.green.max(set.green),
            red: acc.red.max(set.red),
        },
    )
}

fn power_of_set(s: &Set) -> usize {
    s.blue * s.green * s.red
}

fn run_part_one(puzzle_input: &str) {
    let result = puzzle_input
        .lines()
        .filter_map(|line| parse_game(&line))
        .filter(|game| {
            is_game_possible(
                game,
                &Set {
                    red: 12,
                    green: 13,
                    blue: 14,
                },
            )
        })
        .map(|game| game.id)
        .sum::<usize>();

    println!("Part 1: {}", result);
}

fn run_part_two(puzzle_input: &str) {
    let result = puzzle_input
        .lines()
        .filter_map(|line| parse_game(&line))
        .map(|game| fewest_possible_cubes(&game))
        .map(|set| power_of_set(&set))
        .sum::<usize>();

    println!("Part 2: {}", result);
}

pub fn run(puzzle_input: &str) {
    run_part_one(&puzzle_input);
    run_part_two(&puzzle_input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let game = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert!(game.is_some());
        let game = game.unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(
            game.sets,
            vec![
                Set {
                    blue: 3,
                    red: 4,
                    green: 0
                },
                Set {
                    blue: 6,
                    red: 1,
                    green: 2
                },
                Set {
                    blue: 0,
                    red: 0,
                    green: 2
                }
            ]
        );
        let game = parse_game("Game 10: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        assert_eq!(game.id, 10);
    }
}
