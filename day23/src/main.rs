use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)?;
    let input = file_contents.as_str();

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '>' => Self::Slope(Direction::Right),
            'v' => Self::Slope(Direction::Down),
            '<' => Self::Slope(Direction::Left),
            '^' => Self::Slope(Direction::Up),
            _ => panic!("invalid tile"),
        }
    }
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().map(|c| Tile::from(c)).collect()
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn dfs(grid: &Vec<Vec<Tile>>, (row, col): (usize, usize), part2: bool) -> usize {
    let rows = grid.len();
    let columns = grid[0].len();
    let mut seen = vec![vec![false; columns]; rows];
    let mut max_dist = 0;

    fn _dfs(
        grid: &Vec<Vec<Tile>>,
        seen: &mut Vec<Vec<bool>>,
        (row, col): (usize, usize),
        dist: usize,
        max_dist: &mut usize,
        part2: bool,
    ) {
        if row == grid.len() - 1 {
            *max_dist = (*max_dist).max(dist);
        }
        let neighbours = match grid[row][col] {
            _ if part2 => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
            Tile::Path => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
            Tile::Slope(Direction::Right) => [(0, 1)].as_slice(),
            Tile::Slope(Direction::Down) => [(1, 0)].as_slice(),
            Tile::Slope(Direction::Left) => [(0, -1)].as_slice(),
            Tile::Slope(Direction::Up) => [(-1, 0)].as_slice(),
            _ => unreachable!(),
        };
        for &(dr, dc) in neighbours {
            let rr = (row as isize + dr) as usize;
            let cc = (col as isize + dc) as usize;
            let Some(&tile) = grid.get(rr).and_then(|row| row.get(cc)) else {
                continue;
            };
            if tile == Tile::Forest || seen[rr][cc] {
                continue;
            }
            seen[rr][cc] = true;
            _dfs(grid, seen, (rr, cc), dist + 1, max_dist, part2);
            seen[rr][cc] = false;
        }
    }

    _dfs(grid, &mut seen, (row, col), 0, &mut max_dist, part2);
    max_dist
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    dfs(&grid, (0, 1), false)
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    dfs(&grid, (0, 1), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 94)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 154)
    }
}
