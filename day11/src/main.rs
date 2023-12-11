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

#[derive(Debug, PartialEq, Eq)]
enum Row {
    Expand,
    Regular(Vec<Cell>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Expand,
    Galaxy,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

fn manhattan_dist(a: Coord, b: Coord) -> usize {
    b.x.abs_diff(a.x) + b.y.abs_diff(a.y)
}

fn parse_universe(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn expand_universe(map: &Vec<Vec<char>>) -> Vec<Row> {
    let cols = map[0].len();
    // Pass 1. Determine empty columns.
    let mut empty_columns: Vec<bool> = vec![true; cols];
    for row in map {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                empty_columns[x] = false;
            }
        }
    }
    // Pass 2. Now we can expand the universe.
    let mut universe: Vec<Row> = Vec::with_capacity(cols * 2);
    for row in map {
        let mut empty_row = true;
        let mut universe_row: Vec<Cell> = Vec::with_capacity(cols * 2);
        for (x, &c) in row.iter().enumerate() {
            if empty_columns[x] {
                universe_row.push(Cell::Expand);
            } else {
                if c != '.' {
                    empty_row = false;
                    universe_row.push(Cell::Galaxy);
                } else {
                    universe_row.push(Cell::Empty);
                }
            }
        }
        if empty_row {
            universe.push(Row::Expand);
        } else {
            universe.push(Row::Regular(universe_row));
        }
    }
    universe
}

fn get_galaxies(input: &str, expand_size: usize) -> Vec<Coord> {
    let universe = expand_universe(&parse_universe(input));
    let mut galaxies: Vec<Coord> = Vec::new();
    let mut y = 0;
    for row in universe {
        match row {
            Row::Expand => y += expand_size,
            Row::Regular(cells) => {
                let mut x = 0;
                for cell in cells {
                    match cell {
                        Cell::Empty => x += 1,
                        Cell::Expand => x += expand_size,
                        Cell::Galaxy => {
                            galaxies.push(Coord { x, y });
                            x += 1;
                        }
                    }
                }
                y += 1;
            }
        }
    }
    galaxies
}

fn combinations(coords: &Vec<Coord>) -> Vec<(Coord, Coord)> {
    let mut combinations = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            combinations.push((coords[i], coords[j]));
        }
    }
    combinations
}

fn sum_shortest_paths(input: &str, expand_size: usize) -> usize {
    let galaxies = get_galaxies(input, expand_size);
    let mut total = 0;
    for (a, b) in combinations(&galaxies) {
        total += manhattan_dist(a, b);
    }
    total
}

fn part1(input: &str) -> usize {
    sum_shortest_paths(input, 2)
}

fn part2(input: &str) -> usize {
    sum_shortest_paths(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_dist() {
        assert_eq!(
            manhattan_dist(Coord { x: 1, y: 6 }, Coord { x: 5, y: 11 }),
            9
        );
        assert_eq!(
            manhattan_dist(Coord { x: 4, y: 0 }, Coord { x: 9, y: 10 }),
            15
        );
        assert_eq!(
            manhattan_dist(Coord { x: 0, y: 2 }, Coord { x: 12, y: 7 }),
            17
        );
        assert_eq!(
            manhattan_dist(Coord { x: 0, y: 11 }, Coord { x: 5, y: 11 }),
            5
        );
    }

    fn parse_expanded(input: &str) -> Vec<Row> {
        input
            .lines()
            .map(|line| {
                if line == ">" {
                    Row::Expand
                } else {
                    Row::Regular(
                        line.chars()
                            .map(|c| match c {
                                'v' => Cell::Expand,
                                '.' => Cell::Empty,
                                _ => Cell::Galaxy,
                            })
                            .collect(),
                    )
                }
            })
            .collect()
    }

    #[test]
    fn test_expand() {
        let map = parse_universe(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        let expected = parse_expanded(
            "..v#.v..v.
..v..v.#v.
#.v..v..v.
>
..v..v#.v.
.#v..v..v.
..v..v..v#
>
..v..v.#v.
#.v.#v..v.",
        );
        let expanded = expand_universe(&map);
        assert_eq!(expanded, expected);
    }

    fn find_galaxies(map: &Vec<Vec<char>>) -> Vec<Coord> {
        let mut galaxies: Vec<Coord> = Vec::new();
        for (y, row) in map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c != '.' {
                    galaxies.push(Coord { x, y });
                }
            }
        }
        galaxies
    }

    #[test]
    fn test_get_galaxies() {
        let expected = find_galaxies(&parse_universe(
            "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......",
        ));
        let actual = get_galaxies(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            2,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            374
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            sum_shortest_paths(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                10
            ),
            1030
        )
    }
}
