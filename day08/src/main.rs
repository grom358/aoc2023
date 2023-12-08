use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
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

enum Direction {
    Left,
    Right,
}

struct Node {
    name: String,
    left: String,
    right: String,
}

fn parse_direction(c: char) -> Direction {
    match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    }
}

lazy_static! {
    static ref CONNECTION_REGEX: Regex =
        Regex::new(r"(\S+) = \((\S+), (\S+)\)").expect("Failed to compile regex");
}

fn parse_node(line: &str) -> Node {
    let captures = CONNECTION_REGEX.captures(line).unwrap();
    let name = captures[1].to_string();
    let left = captures[2].to_string();
    let right = captures[3].to_string();
    Node { name, left, right }
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let directions = parts[0].chars().map(|c| parse_direction(c)).collect();
    let network: Vec<Node> = parts[1].lines().map(|line| parse_node(line)).collect();
    let mut lookup: HashMap<String, Node> = HashMap::new();
    for node in network {
        lookup.insert(node.name.clone(), node);
    }
    (directions, lookup)
}

fn part1(input: &str) -> u32 {
    let (directions, lookup) = parse_input(input);
    let directions_iter = directions.iter().cycle();

    let mut position: String = String::from("AAA");
    let mut steps = 0;
    for direction in directions_iter {
        if position == "ZZZ" {
            break;
        }
        let node = lookup.get(&position).unwrap();
        let goto = match direction {
            Direction::Left => node.left.clone(),
            Direction::Right => node.right.clone(),
        };
        position = goto;
        steps += 1;
    }
    steps
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(numbers: Vec<usize>) -> usize {
    numbers
        .iter()
        .fold(1, |lcm, &num| lcm * num / gcd(lcm, num))
}

fn part2(input: &str) -> usize {
    let (directions, lookup) = parse_input(input);

    let positions: Vec<String> = lookup
        .keys()
        .filter(|&key| key.ends_with('A'))
        .cloned()
        .collect();

    let mut path_steps: Vec<usize> = Vec::with_capacity(positions.len());

    for position in &positions {
        let mut pos = position.clone();
        let mut steps = 0;
        for direction in directions.iter().cycle() {
            if pos.ends_with('Z') {
                break;
            }
            let node = lookup.get(&pos).unwrap();
            let goto = match direction {
                Direction::Left => node.left.clone(),
                Direction::Right => node.right.clone(),
            };
            pos = goto;
            steps += 1;
        }
        path_steps.push(steps);
    }

    lcm(path_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );

        assert_eq!(
            part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        )
    }
}
