use std::collections::HashSet;
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct BeamPath {
    pos: Coord,
    direction: Direction,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Forward,
    Backward,
    Vertical,
    Horizontal,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::Forward,
            '\\' => Tile::Backward,
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            t => panic!("Invalid tile {}", t),
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Forward => '/',
            Tile::Backward => '\\',
            Tile::Vertical => '|',
            Tile::Horizontal => '-',
        }
    }
}

fn parse_layout(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| Tile::from(c)).collect())
        .collect()
}

fn get_beam_direction(input: Direction, tile: Tile) -> Vec<Direction> {
    let mut outputs: Vec<Direction> = Vec::with_capacity(2);
    match (input, tile) {
        (dir, Tile::Empty) => outputs.push(dir),
        (Direction::Right, Tile::Forward) => outputs.push(Direction::Up),
        (Direction::Right, Tile::Backward) => outputs.push(Direction::Down),
        (Direction::Right, Tile::Horizontal) => outputs.push(Direction::Right),
        (Direction::Left, Tile::Forward) => outputs.push(Direction::Down),
        (Direction::Left, Tile::Backward) => outputs.push(Direction::Up),
        (Direction::Left, Tile::Horizontal) => outputs.push(Direction::Left),
        (Direction::Down, Tile::Forward) => outputs.push(Direction::Left),
        (Direction::Down, Tile::Backward) => outputs.push(Direction::Right),
        (Direction::Down, Tile::Vertical) => outputs.push(Direction::Down),
        (Direction::Up, Tile::Forward) => outputs.push(Direction::Right),
        (Direction::Up, Tile::Backward) => outputs.push(Direction::Left),
        (Direction::Up, Tile::Vertical) => outputs.push(Direction::Up),
        (Direction::Right, Tile::Vertical) => {
            outputs.push(Direction::Up);
            outputs.push(Direction::Down);
        }
        (Direction::Left, Tile::Vertical) => {
            outputs.push(Direction::Up);
            outputs.push(Direction::Down);
        }
        (Direction::Down, Tile::Horizontal) => {
            outputs.push(Direction::Right);
            outputs.push(Direction::Left);
        }
        (Direction::Up, Tile::Horizontal) => {
            outputs.push(Direction::Right);
            outputs.push(Direction::Left);
        }
    }
    outputs
}

fn bound_inc(dir: Direction, pos: Coord, rows: usize, cols: usize) -> Option<Coord> {
    match dir {
        Direction::Right => {
            if pos.x + 1 < cols {
                Some(Coord {
                    x: pos.x + 1,
                    y: pos.y,
                })
            } else {
                None
            }
        }
        Direction::Down => {
            if pos.y + 1 < rows {
                Some(Coord {
                    x: pos.x,
                    y: pos.y + 1,
                })
            } else {
                None
            }
        }
        Direction::Left => {
            if pos.x > 0 {
                Some(Coord {
                    x: pos.x - 1,
                    y: pos.y,
                })
            } else {
                None
            }
        }
        Direction::Up => {
            if pos.y > 0 {
                Some(Coord {
                    x: pos.x,
                    y: pos.y - 1,
                })
            } else {
                None
            }
        }
    }
}

fn calc_energized(start: BeamPath, layout: &Vec<Vec<Tile>>) -> usize {
    let mut to_visit: Vec<BeamPath> = Vec::new();
    for dir in get_beam_direction(start.direction, layout[start.pos.y][start.pos.x]) {
        to_visit.push(BeamPath {
            pos: start.pos,
            direction: dir,
        });
    }
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut seen: HashSet<BeamPath> = HashSet::new();
    let rows = layout.len();
    let cols = layout[0].len();
    while to_visit.len() > 0 {
        let bp = to_visit.pop().unwrap();
        if seen.contains(&bp) {
            continue;
        }
        seen.insert(bp);
        visited.insert(bp.pos);
        if let Some(new_pos) = bound_inc(bp.direction, bp.pos, rows, cols) {
            let tile = layout[new_pos.y][new_pos.x];
            for dir in get_beam_direction(bp.direction, tile) {
                to_visit.push(BeamPath {
                    pos: new_pos,
                    direction: dir,
                });
            }
        }
    }
    visited.len()
}

fn part1(input: &str) -> usize {
    let layout = &parse_layout(input);
    let start = BeamPath {
        pos: Coord { x: 0, y: 0 },
        direction: Direction::Right,
    };
    calc_energized(start, &layout)
}

fn part2(input: &str) -> usize {
    let layout = parse_layout(input);
    let rows = layout.len();
    let cols = layout[0].len();
    let mut max_energy = 0;
    let mut possible_starts: Vec<BeamPath> = Vec::with_capacity((rows + 1) * 2 + (cols + 1) * 2);
    possible_starts.push(BeamPath {
        pos: Coord { x: 0, y: 0 },
        direction: Direction::Right,
    });
    possible_starts.push(BeamPath {
        pos: Coord { x: 0, y: 0 },
        direction: Direction::Down,
    });
    possible_starts.push(BeamPath {
        pos: Coord { x: cols - 1, y: 0 },
        direction: Direction::Left,
    });
    possible_starts.push(BeamPath {
        pos: Coord { x: cols - 1, y: 0 },
        direction: Direction::Down,
    });
    possible_starts.push(BeamPath {
        pos: Coord { x: 0, y: rows - 1 },
        direction: Direction::Right,
    });
    possible_starts.push(BeamPath {
        pos: Coord { x: 0, y: rows - 1 },
        direction: Direction::Up,
    });
    possible_starts.push(BeamPath {
        pos: Coord {
            x: cols - 1,
            y: rows - 1,
        },
        direction: Direction::Left,
    });
    possible_starts.push(BeamPath {
        pos: Coord {
            x: cols - 1,
            y: rows - 1,
        },
        direction: Direction::Up,
    });
    for x in 1..cols - 1 {
        possible_starts.push(BeamPath {
            pos: Coord { x, y: 0 },
            direction: Direction::Down,
        });
        possible_starts.push(BeamPath {
            pos: Coord { x, y: rows - 1 },
            direction: Direction::Up,
        });
    }
    for y in 1..rows - 1 {
        possible_starts.push(BeamPath {
            pos: Coord { x: 0, y },
            direction: Direction::Right,
        });
        possible_starts.push(BeamPath {
            pos: Coord { x: cols - 1, y },
            direction: Direction::Left,
        });
    }
    for start in possible_starts {
        let energy = calc_energized(start, &layout);
        if energy > max_energy {
            max_energy = energy;
        }
    }
    max_energy
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 46)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 51)
    }
}
