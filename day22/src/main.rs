use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)?;
    let input = file_contents.as_str();

    let (part1, part2) = run(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

type BrickId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    a: Coord,
    b: Coord,
    id: BrickId,
}

impl Brick {
    fn down(&self) -> Brick {
        let mut result = self.clone();
        result.a.z -= 1;
        result.b.z -= 1;
        result
    }

    fn positions(&self) -> Vec<Coord> {
        let mut results = Vec::new();
        for x in self.a.x..=self.b.x {
            for y in self.a.y..=self.b.y {
                for z in self.a.z..=self.b.z {
                    results.push(Coord { x, y, z });
                }
            }
        }
        results
    }
}

fn parse_coord(str_coord: &str) -> Coord {
    let nums: Vec<usize> = str_coord.split(',').map(|s| s.parse().unwrap()).collect();
    Coord {
        x: nums[0],
        y: nums[1],
        z: nums[2],
    }
}

fn parse_line(line_no: usize, line: &str) -> Brick {
    let ends: Vec<Coord> = line
        .split('~')
        .map(|str_coord| parse_coord(str_coord))
        .collect();
    Brick {
        a: ends[0],
        b: ends[1],
        id: line_no,
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .map(|(line_no, line)| parse_line(line_no, line))
        .collect()
}

fn simulate(bricks: &Vec<Brick>) -> (usize, usize) {
    let mut occupied = HashMap::new();
    let mut fallen = Vec::new();

    // Simulate bricks falling.
    let mut bricks = bricks.clone();
    bricks.sort_by_key(|brick| brick.a.z);
    for brick in bricks {
        let mut brick = brick;
        loop {
            let next = brick.down();
            let can_move = next.a.z > 0
                && !next
                    .positions()
                    .iter()
                    .any(|pos| occupied.contains_key(pos));
            if can_move {
                brick = next;
            } else {
                for pos in brick.positions() {
                    occupied.insert(pos, brick);
                }
                fallen.push(brick);
                break;
            }
        }
    }

    // Calculate which bricks are above and below each other.
    let mut above = HashMap::new();
    let mut below = HashMap::new();
    for &brick in fallen.iter() {
        let in_this_brick: HashSet<Coord> = brick.positions().into_iter().collect();
        for pos in brick.down().positions() {
            if let Some(&occupied_brick) = occupied.get(&pos) {
                if !in_this_brick.contains(&pos) {
                    above
                        .entry(occupied_brick.id)
                        .or_insert(HashSet::new())
                        .insert(brick.id);
                    below
                        .entry(brick.id)
                        .or_insert(HashSet::new())
                        .insert(occupied_brick.id);
                }
            }
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for brick in fallen.iter().cloned() {
        let would_fall = what_if(&brick, &above, &below);
        if would_fall == 1 {
            part1 += 1
        }
        part2 += would_fall - 1
    }
    (part1, part2)
}

// How many bricks fall if we disintegrate a brick.
fn what_if(
    disintegrated: &Brick,
    above: &HashMap<BrickId, HashSet<BrickId>>,
    below: &HashMap<BrickId, HashSet<BrickId>>,
) -> usize {
    fn falls(
        brick: usize,
        falling: &mut HashSet<BrickId>,
        above: &HashMap<BrickId, HashSet<BrickId>>,
        below: &HashMap<BrickId, HashSet<BrickId>>,
    ) {
        if falling.contains(&brick) {
            return;
        }
        falling.insert(brick);
        if let Some(parents) = above.get(&brick) {
            for &parent in parents {
                if let Some(children) = below.get(&parent) {
                    if children.difference(&falling).count() == 0 {
                        // If everything below the parent is falling, so is the parent.
                        falls(parent, falling, above, below);
                    }
                }
            }
        }
    }

    let mut falling = HashSet::new();
    falls(disintegrated.id, &mut falling, above, below);
    falling.len()
}

fn run(input: &str) -> (usize, usize) {
    let bricks = parse(input);
    simulate(&bricks)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_run() {
        assert_eq!(run(SAMPLE), (5, 7))
    }
}
