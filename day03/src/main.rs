use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cell {
    Number {
        row: usize,
        start_column: usize,
        end_column: usize,
        value: u32,
    },
    Symbol {
        row: usize,
        column: usize,
    },
    Empty,
}

lazy_static! {
    static ref SCHEMATIC_REGEX: Regex =
        Regex::new(r"(\d+|[^\.])").expect("Failed to compile regex");
}

fn process_line(row: usize, line: &str) -> Vec<Cell> {
    let items: Vec<Cell> = SCHEMATIC_REGEX
        .find_iter(line)
        .map(|mat| {
            let value = mat.as_str();
            if value.chars().next().map_or(false, |c| c.is_digit(10)) {
                let num: u32 = value.parse().unwrap();
                Cell::Number {
                    row,
                    start_column: mat.start(),
                    end_column: mat.end() - 1,
                    value: num,
                }
            } else {
                Cell::Symbol {
                    row,
                    column: mat.start(),
                }
            }
        })
        .collect();
    let n = line.len();
    let mut row: Vec<Cell> = Vec::with_capacity(n);
    for _ in 0..n {
        row.push(Cell::Empty);
    }
    for item in items {
        match item {
            Cell::Number {
                start_column,
                end_column,
                ..
            } => {
                // clone the number into every position it occupies.
                for i in start_column..=end_column {
                    row[i] = item.clone();
                }
            }
            Cell::Symbol { column, .. } => {
                row[column] = item.clone();
            }
            Cell::Empty => {
                // do nothing
            }
        }
    }
    row
}

fn process_lines(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| process_line(row, line))
        .collect()
}

fn parts_for_symbol(
    row_count: usize,
    column_count: usize,
    grid: &Vec<Vec<Cell>>,
    cell: Cell,
    parts: &mut HashSet<Cell>,
) {
    if let Cell::Symbol { row, column, .. } = cell {
        let first_col = if column > 0 { column - 1 } else { 0 };
        let last_col = std::cmp::min(column_count - 1, column + 1);
        if row > 0 {
            for col in first_col..=last_col {
                let neighbour = &grid[row - 1][col];
                if let Cell::Number { .. } = neighbour {
                    parts.insert(neighbour.clone());
                }
            }
        }
        for col in first_col..=last_col {
            let neighbour = &grid[row][col];
            if let Cell::Number { .. } = neighbour {
                parts.insert(neighbour.clone());
            }
        }
        if row < row_count - 1 {
            for col in first_col..=last_col {
                let neighbour = &grid[row + 1][col];
                if let Cell::Number { .. } = neighbour {
                    parts.insert(neighbour.clone());
                }
            }
        }
    }
}

fn part1(input: &str) -> u32 {
    let grid = process_lines(input);
    let mut parts = HashSet::new();
    for row in &grid {
        for cell in row {
            parts_for_symbol(grid.len(), row.len(), &grid, cell.clone(), &mut parts);
        }
    }
    let mut total = 0;
    for part in parts {
        if let Cell::Number { value, .. } = part {
            total += value;
        }
    }
    total
}

fn part2(input: &str) -> u32 {
    let grid = process_lines(input);
    let mut total = 0;
    for row in &grid {
        for cell in row {
            let mut parts = HashSet::new();
            parts_for_symbol(grid.len(), row.len(), &grid, cell.clone(), &mut parts);
            if parts.len() == 2 {
                let mut gear_ratio = 1;
                for part in parts {
                    if let Cell::Number { value, .. } = part {
                        gear_ratio *= value;
                    }
                }
                total += gear_ratio;
            }
        }
    }
    total
}

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)?;
    let input = file_contents.as_str();

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            467835
        )
    }
}
