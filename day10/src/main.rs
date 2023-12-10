use std::fs;
use std::io;
use std::collections::HashSet;

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)?;
    let input = file_contents.as_str();

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));

    Ok(())
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pipe {
    NorthSouth,
    WestEast,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start, // pipe type is hidden.
}

fn parse_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::NorthSouth,
        '-' => Pipe::WestEast,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::Ground,
        'S' => Pipe::Start,
        _ => panic!("Invalid input"),
    }
}

fn parse_line(line: &str) -> Vec<Pipe> {
    line.chars().map(|c| parse_pipe(c)).collect()
}

fn parse_grid_lines(input: &str) -> Vec<Vec<Pipe>> {
    input.lines()
        .map(|line| parse_line(line))
        .collect()
}

fn calculate_pipe(north: Pipe, east: Pipe, south: Pipe, west: Pipe) -> Option<Pipe> {
    let north_connected = north == Pipe::SouthEast || north == Pipe::SouthWest || north == Pipe::NorthSouth;
    let south_connected = south == Pipe::NorthEast || south == Pipe::NorthWest || south == Pipe::NorthSouth;
    let east_connected = east == Pipe::WestEast || east == Pipe::NorthWest || east == Pipe::SouthWest;
    let west_connected = west == Pipe::WestEast || west == Pipe::NorthEast || west == Pipe::SouthEast;

    if north_connected && south_connected {
        Some(Pipe::NorthSouth)
    } else if north_connected && east_connected {
        Some(Pipe::NorthEast)
    } else if north_connected && west_connected {
        Some(Pipe::NorthWest)
    } else if south_connected && east_connected {
        Some(Pipe::SouthEast)
    } else if south_connected && west_connected {
        Some(Pipe::SouthWest)
    } else if west_connected && east_connected {
        Some(Pipe::WestEast)
    } else {
        None
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    start_pos: Coord,
    start_pipe: Pipe,
    cells: Vec<Vec<Pipe>>,
}

fn find_start(cells: &Vec<Vec<Pipe>>) -> Option<Coord> {
    for (row_idx, row) in cells.iter().enumerate() {
        for (col_idx, &pipe) in row.iter().enumerate() {
            if pipe == Pipe::Start {
                return Some(Coord { x: col_idx, y: row_idx })
            }
        }
    }
    None
}

fn parse_grid(input: &str) -> Grid {
    let mut cells = parse_grid_lines(input);
    let start_pos = find_start(&cells).unwrap();
    let row_len = cells.len();
    let col_len = cells[0].len();
    let start_row = start_pos.y;
    let start_col = start_pos.x;
    let north = if start_row > 0 { cells[start_row - 1][start_col] } else { Pipe::Ground };
    let east = if start_col + 1 < col_len { cells[start_row][start_col + 1] } else { Pipe::Ground };
    let south = if start_row + 1 < row_len { cells[start_row + 1][start_col] } else { Pipe::Ground };
    let west = if start_col > 0 { cells[start_row][start_col - 1] } else { Pipe::Ground };
    let start_pipe = calculate_pipe(north, east, south, west).unwrap();
    cells[start_pos.y][start_pos.x] = start_pipe;
    Grid { start_pos, start_pipe, cells }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn apply_direction(pos: Coord, dir: Direction) -> Coord {
    match dir {
        Direction::North => Coord { y: pos.y - 1, x: pos.x },
        Direction::South => Coord { y: pos.y + 1, x: pos.x },
        Direction::East => Coord { x: pos.x + 1, y: pos.y },
        Direction::West => Coord { x: pos.x - 1, y: pos.y },
    }
}

fn start_directions(start_pipe: Pipe) -> (Direction, Direction) {
    match start_pipe {
        Pipe::NorthSouth => (Direction::North, Direction::South),
        Pipe::WestEast => (Direction::West, Direction::East),
        Pipe::NorthEast => (Direction::North, Direction::East),
        Pipe::NorthWest => (Direction::North, Direction::West),
        Pipe::SouthEast => (Direction::South, Direction::East),
        Pipe::SouthWest => (Direction::South, Direction::West),
        _ => panic!("Invalid start pipe"),
    }
}

fn reverse_direction(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn next_direction(from: Direction, pipe: Pipe) -> Direction {
    match pipe {
       Pipe::NorthSouth => {
           match from {
               Direction::North => Direction::South,
               Direction::South => Direction::North,
               _ => panic!("Invalid input"),
           }
       },
       Pipe::WestEast => {
           match from {
               Direction::West => Direction::East,
               Direction::East => Direction::West,
               _ => panic!("Invalid input"),
           }
       },
       Pipe::NorthWest => {
           match from {
               Direction::North => Direction::West,
               Direction::West => Direction::North,
               _ => panic!("Invalid input"),
           }
       },
       Pipe::NorthEast => {
           match from {
               Direction::North => Direction::East,
               Direction::East => Direction::North,
               _ => panic!("Invalid input"),
           }
       },
       Pipe::SouthWest => {
           match from {
               Direction::South => Direction::West,
               Direction::West => Direction::South,
               _ => panic!("Invalid input"),
           }
       },
       Pipe::SouthEast => {
           match from {
               Direction::South => Direction::East,
               Direction::East => Direction::South,
               _ => panic!("Invalid input"),
           }
       },
       _ => panic!("Invalid input"),
   }
}

fn find_loop(grid: &Grid) -> Vec<Coord> {
    let mut path = Vec::new();
    path.push(grid.start_pos);
    let (dir1, dir2) = start_directions(grid.start_pipe);
    let mut from1 = reverse_direction(dir1);
    let mut from2 = reverse_direction(dir2);
    let mut pos1 = apply_direction(grid.start_pos, dir1);
    let mut pos2 = apply_direction(grid.start_pos, dir2);
    while pos1 != pos2 {
        path.push(pos1);
        path.push(pos2);
        let pipe1 = grid.cells[pos1.y][pos1.x];
        let pipe2 = grid.cells[pos2.y][pos2.x];
        from1 = next_direction(from1, pipe1);
        from2 = next_direction(from2, pipe2);
        pos1 = apply_direction(pos1, from1);
        pos2 = apply_direction(pos2, from2);
        from1 = reverse_direction(from1);
        from2 = reverse_direction(from2);
    }
    path.push(pos1);
    path
}

fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    find_loop(&grid).len() / 2
}

fn is_inside_polygon(pos: Coord, polygon: &HashSet<Coord>, cols: usize, cells: &Vec<Vec<Pipe>>) -> bool {
    let mut intersections = 0;
    for x in pos.x..cols {
        let test_pos = Coord { x, y: pos.y };
        if polygon.contains(&test_pos) {
            let pipe = cells[test_pos.y][test_pos.x];
            if pipe == Pipe::NorthSouth || pipe == Pipe::NorthEast || pipe == Pipe::NorthWest {
                intersections += 1;
            }
        }
    }
    intersections % 2 == 1
}

fn part2(input: &str) -> usize {
    let grid = parse_grid(input);
    let path = find_loop(&grid);
    let path_set: HashSet<_> = path.into_iter().collect();
    let rows = grid.cells.len();
    let cols = grid.cells[0].len();
    let mut enclosed = 0;
    for y in 0..rows {
        for x in 0..cols {
            let pos = Coord { x, y };
            if !path_set.contains(&pos) && is_inside_polygon(pos, &path_set, cols, &grid.cells) {
                enclosed += 1;
            }
        }
    }
    enclosed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_lines() {
        assert_eq!(parse_grid_lines(".....
.F-7.
.|.|.
.L-J.
....."), vec!(
            vec!(Pipe::Ground, Pipe::Ground, Pipe::Ground, Pipe::Ground, Pipe::Ground),
            vec!(Pipe::Ground, Pipe::SouthEast, Pipe::WestEast, Pipe::SouthWest, Pipe::Ground),
            vec!(Pipe::Ground, Pipe::NorthSouth, Pipe::Ground, Pipe::NorthSouth, Pipe::Ground),
            vec!(Pipe::Ground, Pipe::NorthEast, Pipe::WestEast, Pipe::NorthWest, Pipe::Ground),
            vec!(Pipe::Ground, Pipe::Ground, Pipe::Ground, Pipe::Ground, Pipe::Ground),
        ))
    }

    fn start_pipe(input: &str) -> Pipe {
        let grid = parse_grid(input);
        grid.start_pipe
    }

    #[test]
    fn test_calculate_start() {
        assert_eq!(start_pipe("...
.|.
.S.
.|."), Pipe::NorthSouth);
        assert_eq!(start_pipe("...
.|.
.S-"), Pipe::NorthEast);
        assert_eq!(start_pipe("...
.|.
-S.
..."), Pipe::NorthWest);
        assert_eq!(start_pipe("...
-S.
.|."), Pipe::SouthWest);
        assert_eq!(start_pipe("...
.S-
.|."), Pipe::SouthEast);
        assert_eq!(start_pipe("...
-S-
..."), Pipe::WestEast);
        assert_eq!(start_pipe(".....
.S-7.
.|.|.
.L-J.
....."), Pipe::SouthEast);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"),
            8
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."), 4);

        assert_eq!(part2(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."), 8);

        assert_eq!(part2("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"), 10);
    }
}
