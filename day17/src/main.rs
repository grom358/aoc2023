use std::collections::{BinaryHeap, HashMap};
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

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| parse_line(line)).collect()
}

// Directions as (row, col).
type Direction = (isize, isize);
const RIGHT: Direction = (0, 1);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);
const UP: Direction = (-1, 0);
const DIRECTIONS: [Direction; 4] = [RIGHT, DOWN, LEFT, UP];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PathNode {
    row: usize,
    col: usize,
    dir: Direction,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    path_node: PathNode,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Vec<Vec<u8>>, min_step: isize, max_step: isize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let goal = (rows - 1, cols - 1);
    let mut dists: HashMap<PathNode, usize> = HashMap::new();
    let mut queue = BinaryHeap::from_iter([State {
        cost: 0,
        path_node: PathNode {
            row: 0,
            col: 0,
            dir: (0, 0),
        },
    }]);
    while let Some(State { cost, path_node }) = queue.pop() {
        let PathNode { row, col, dir } = path_node;
        if (row, col) == goal {
            return cost;
        }
        if dists.get(&path_node).is_some_and(|&c| cost > c) {
            continue;
        }
        for (dr, dc) in DIRECTIONS {
            if dir == (dr, dc) || dir == (-dr, -dc) {
                continue;
            }
            let mut next_cost = cost;
            for dist in 1..=max_step {
                let rr = (row as isize + dr * dist) as usize;
                let cc = (col as isize + dc * dist) as usize;
                if rr >= rows || cc >= cols {
                    continue;
                }
                next_cost += grid[rr][cc] as usize;
                let key = PathNode {
                    row: rr,
                    col: cc,
                    dir: (dr, dc),
                };
                if min_step <= dist && next_cost < *dists.get(&key).unwrap_or(&usize::MAX) {
                    dists.insert(key, next_cost);
                    queue.push(State {
                        cost: next_cost,
                        path_node: key,
                    });
                }
            }
        }
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    dijkstra(&grid, 1, 3)
}

fn part2(input: &str) -> usize {
    let grid = parse_grid(input);
    dijkstra(&grid, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 102)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 94);
        assert_eq!(
            part2(
                "111111111111
999999999991
999999999991
999999999991
999999999991"
            ),
            71
        )
    }
}
