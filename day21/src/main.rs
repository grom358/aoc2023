use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)?;
    let input = file_contents.as_str();

    println!("Part 1: {}", part1(input, 64));
    println!("Part 2: {}", part2(input));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Garden,
            '#' => Tile::Rock,
            'S' => Tile::Garden,
            _ => panic!("invalid tile"),
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, (isize, isize)) {
    let mut start_pos = (0, 0);
    let mut map = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut map_row = Vec::new();
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start_pos = (row as isize, col as isize);
            }
            map_row.push(Tile::from(c));
        }
        map.push(map_row);
    }
    (map, start_pos)
}

/**
 * Tests if (row, col) is a garden with infinite wrapping of map.
 */
fn is_garden(map: &Vec<Vec<Tile>>, row: isize, col: isize) -> bool {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;
    let r = row.rem_euclid(rows);
    let c = col.rem_euclid(cols);
    map[r as usize][c as usize] == Tile::Garden
}

fn bfs_steps(map: &Vec<Vec<Tile>>, start: (isize, isize), max_steps: [usize; 3]) -> [usize; 3] {
    let mut seen_states = HashSet::new();
    let mut queue = VecDeque::new();
    let mut results = vec![HashSet::new(); 3];

    {
        let (r, c) = start;
        queue.push_back((r, c, 0));
    }

    while let Some((row, col, steps)) = queue.pop_front() {
        for i in 0..3 {
            if steps == max_steps[i] {
                results[i].insert((row, col));
            }
        }
        if seen_states.contains(&(row, col, steps)) {
            continue;
        }
        seen_states.insert((row, col, steps));

        if steps + 1 <= max_steps[2] {
            for (dr, dc) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let r = row + dr;
                let c = col + dc;
                if is_garden(&map, r, c) {
                    queue.push_back((r, c, steps + 1));
                }
            }
        }
    }

    results
        .iter()
        .map(|result| result.len())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

fn part1(input: &str, steps: usize) -> usize {
    let (map, start) = parse(input);
    let max_steps = [0, 0, steps];
    bfs_steps(&map, start, max_steps)[2]
}

fn part2(input: &str) -> usize {
    let (map, start) = parse(input);
    // Assumes there are no rocks in the same row/column and
    // the edges are empty. Therefore as we expand out the
    // number of steps is quadratic.
    let progression = bfs_steps(&map, start, [65, 65 + 131, 65 + 131 * 2]);
    let x = (26501365 - 65) / 131;
    let (a, b, c) = quadratic_fit(progression);
    a * x * x + b * x + c
}

fn quadratic_fit(progression: [usize; 3]) -> (usize, usize, usize) {
    let [y1, y2, y3] = progression;
    let a = (y3 - 2 * y2 + y1) / 2;
    let b = (y2 - y1) - a;
    let c = y1;

    (a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE, 6), 16)
    }
}
