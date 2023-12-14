use std::fs;
use std::collections::{HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::io;

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)?;
    let input = file_contents.as_str();

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));

    Ok(())
}

#[allow(dead_code)]
fn grid_str(grid: &Vec<Vec<Tile>>) -> String {
    let mut result = String::new();
    for row in grid {
        for &tile in row {
            result.push(tile.into());
        }
        result.push('\n');
    }
    result
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Tile {
    Round,
    Cube,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'O' => Tile::Round,
            '#' => Tile::Cube,
            '.' => Tile::Empty,
            t => panic!("Invalid tile {}", t),
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Round => 'O',
            Tile::Cube => '#',
            Tile::Empty => '.',
        }
    }
}

fn parse_grid(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| Tile::from(c)).collect())
        .collect()
}

fn tilt_north(grid: &mut Vec<Vec<Tile>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    for y in 1..rows {
        for x in 0..cols {
            if grid[y][x] == Tile::Round {
                let mut new_y = y;
                while new_y > 0 {
                    let above = grid[new_y - 1][x];
                    if above == Tile::Cube || above == Tile::Round {
                        break;
                    }
                    new_y -= 1;
                }
                if new_y != y {
                    grid[y][x] = Tile::Empty;
                    grid[new_y][x] = Tile::Round;
                }
            }
        }
    }
}

fn score(grid: &Vec<Vec<Tile>>) -> usize {
    let rows = grid.len();
    let mut score = 0;
    for (y, row) in grid.iter().enumerate() {
        let per_rock = rows - y;
        let round_count = row.iter().filter(|t| **t == Tile::Round).count();
        score += round_count * per_rock;
    }
    score
}

fn part1(input: &str) -> usize {
    let mut grid = parse_grid(input);
    tilt_north(&mut grid);
    score(&grid)
}

fn tilt_south(grid: &mut Vec<Vec<Tile>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    for y in (0..rows - 1).rev() {
        for x in 0..cols {
            if grid[y][x] == Tile::Round {
                let mut new_y = y;
                while new_y < rows - 1 {
                    let below = grid[new_y + 1][x];
                    if below == Tile::Cube || below == Tile::Round {
                        break;
                    }
                    new_y += 1;
                }
                if new_y != y {
                    grid[y][x] = Tile::Empty;
                    grid[new_y][x] = Tile::Round;
                }
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<Tile>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    for y in 0..rows {
        for x in 1..cols {
            if grid[y][x] == Tile::Round {
                let mut new_x = x;
                while new_x > 0 {
                    let left = grid[y][new_x - 1];
                    if left == Tile::Cube || left == Tile::Round {
                        break;
                    }
                    new_x -= 1;
                }
                if new_x != x {
                    grid[y][x] = Tile::Empty;
                    grid[y][new_x] = Tile::Round;
                }
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<Tile>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    for y in 0..rows {
        for x in (0..cols - 1).rev() {
            if grid[y][x] == Tile::Round {
                let mut new_x = x;
                while new_x < cols - 1 {
                    let right = grid[y][new_x + 1];
                    if right == Tile::Cube || right == Tile::Round {
                        break;
                    }
                    new_x += 1;
                }
                if new_x != x {
                    grid[y][x] = Tile::Empty;
                    grid[y][new_x] = Tile::Round;
                }
            }
        }
    }
}

fn tilt_cycle(grid: &mut Vec<Vec<Tile>>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn hash_grid(grid: &Vec<Vec<Tile>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for row in grid {
        for tile in row {
            tile.hash(&mut hasher);
        }
    }
    hasher.finish()
}

fn part2(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut steps = 0;
    let mut cycle = 0;
    for _ in 0..3 {
        let mut hash_set: HashSet<u64> = HashSet::new();
        cycle = 0;
        loop {
            let hash = hash_grid(&grid);
            if hash_set.contains(&hash) {
                break;
            }
            hash_set.insert(hash);
            tilt_cycle(&mut grid);
            steps += 1;
            cycle += 1;
        }
    }
    let mut remaining = 1_000_000_000 - steps;
    let repeats = remaining / cycle;
    remaining -= repeats * cycle;
    for _ in 0..remaining {
        tilt_cycle(&mut grid);
    }
    score(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_tilt() {
        let mut grid = parse_grid(SAMPLE);
        tilt_north(&mut grid);
        assert_eq!(
            grid,
            parse_grid(
                "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
            )
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 136)
    }

    #[test]
    fn test_cycle() {
        let mut grid = parse_grid(SAMPLE);
        tilt_cycle(&mut grid);
        assert_eq!(
            grid,
            parse_grid(
                ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            )
        );
        tilt_cycle(&mut grid);
        assert_eq!(
            grid,
            parse_grid(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            )
        );
        tilt_cycle(&mut grid);
        assert_eq!(
            grid,
            parse_grid(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 64)
    }
}
