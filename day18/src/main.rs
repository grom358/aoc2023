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
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Op {
    direction: Direction,
    length: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

fn polygon_create(ops: &Vec<Op>) -> Vec<Point> {
    let mut pos = Point { x: 0, y: 0 }; // Start at origin.
    let mut vertices: Vec<Point> = Vec::with_capacity(ops.len() + 1);
    vertices.push(pos);
    for op in ops {
        match op.direction {
            Direction::Right => pos.x += op.length,
            Direction::Down => pos.y += op.length,
            Direction::Left => pos.x -= op.length,
            Direction::Up => pos.y -= op.length,
        }
        vertices.push(pos);
    }
    vertices
}

fn polygon_area(vertices: &Vec<Point>) -> i64 {
    let mut shoelace_area = 0;
    let mut boundary = 0;
    for i in 0..vertices.len() {
        let current = vertices[i];
        let next = vertices[(i + 1) % vertices.len()];
        boundary += (next.x - current.x).abs() + (next.y - current.y).abs();
        shoelace_area += current.x * next.y - next.x * current.y;
    }
    shoelace_area = shoelace_area.abs() / 2;

    // Pick's theorem
    let interior_area = shoelace_area - boundary / 2 + 1;

    interior_area + boundary
}

fn part1_parse_line(line: &str) -> Op {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let direction = match parts[0] {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        _ => panic!("invalid direction"),
    };
    let length = i64::from_str_radix(parts[1], 10).unwrap();
    Op { direction, length }
}

fn part1(input: &str) -> i64 {
    let ops: Vec<Op> = input.lines().map(|line| part1_parse_line(line)).collect();
    let polygon = polygon_create(&ops);
    polygon_area(&polygon)
}

fn part2_parse_line(line: &str) -> Op {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let code = parts[2].trim_matches(|c| c == '#' || c == '(' || c == ')');
    let direction_code = code.chars().last().unwrap();
    let direction = match direction_code {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        c => panic!("invalid direction {c}"),
    };
    let length = i64::from_str_radix(&code[..5], 16).unwrap();
    Op { direction, length }
}

fn part2(input: &str) -> i64 {
    let ops: Vec<Op> = input.lines().map(|line| part2_parse_line(line)).collect();
    let polygon = polygon_create(&ops);
    polygon_area(&polygon)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 62)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 952408144115)
    }
}
