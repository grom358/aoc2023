use std::fs;
use std::io;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)?;
    let input = file_contents.as_str();

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    
    Ok(())
}

struct Cubes {
    blue: u8,
    green: u8,
    red: u8,
}

struct Game {
    game_id: u32,
    samples: Vec<Cubes>,
}

lazy_static! {
    static ref GAME_REGEX: Regex =
        Regex::new(r"Game (\d+): (.*)").expect("Failed to parse game regex");
    static ref SAMPLE_REGEX: Regex =
        Regex::new(r"(\d+) (blue|green|red)").expect("Failed to parse sample regex");
}

fn parse_game(game_line: &str) -> Option<Game> {
    let captures = GAME_REGEX.captures(game_line)?;
    let game_id: u32 = captures[1].parse().ok()?;
    let games_str = &captures[2];

    let mut games = Vec::new();
    for game_str in games_str.split(';') {
        let samples = SAMPLE_REGEX
            .captures_iter(game_str)
            .filter_map(|cap| {
                let quantity: u8 = cap[1].parse().ok()?;
                let color_str = &cap[2];

                match color_str {
                    "blue" => Some(Cubes {
                        blue: quantity,
                        green: 0,
                        red: 0,
                    }),
                    "green" => Some(Cubes {
                        blue: 0,
                        green: quantity,
                        red: 0,
                    }),
                    "red" => Some(Cubes {
                        blue: 0,
                        green: 0,
                        red: quantity,
                    }),
                    _ => None,
                }
            })
            .fold(
                Cubes {
                    blue: 0,
                    green: 0,
                    red: 0,
                },
                |acc, cubes| Cubes {
                    blue: acc.blue + cubes.blue,
                    green: acc.green + cubes.green,
                    red: acc.red + cubes.red,
                },
            );
        games.push(samples);
    }
    Some(Game {
        game_id,
        samples: games,
    })
}

fn is_valid(game: &Game, condition: &Cubes) -> bool {
    for sample in game.samples.iter() {
        if sample.red > condition.red
            || sample.green > condition.green
            || sample.blue > condition.blue
        {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> u32 {
    let condition = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .lines()
        .filter_map(|line| parse_game(line))
        .filter(|game| is_valid(game, &condition))
        .map(|game| game.game_id)
        .sum()
}

fn min_power(game: Game) -> u32 {
    let mut required = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };
    for sample in game.samples {
        required.red = std::cmp::max(required.red, sample.red);
        required.green = std::cmp::max(required.green, sample.green);
        required.blue = std::cmp::max(required.blue, sample.blue);
    }
    required.red as u32 * required.green as u32 * required.blue as u32
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| parse_game(line))
        .map(|game| min_power(game))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        )
    }
}
