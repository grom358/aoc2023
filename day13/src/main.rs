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

fn check_mirror(lines: &Vec<&str>, split_at: usize) -> bool {
    let (first, second) = lines.split_at(split_at);
    let n = std::cmp::min(first.len(), second.len());
    let mirror_first: Vec<&str> = first.iter().rev().cloned().take(n).collect();
    let mirror_second: Vec<&str> = second.iter().cloned().take(n).collect();
    mirror_first == mirror_second
}

fn find_reflection(pattern: &str) -> usize {
    let lines: Vec<&str> = pattern.lines().collect();
    let mut columns_str: Vec<String> = vec![String::new(); lines[0].len()];
    for (x, c) in lines[0].chars().enumerate() {
        columns_str[x].push(c);
    }
    let mut row = 1;
    for window in lines.windows(2) {
        if window[0] == window[1] && check_mirror(&lines, row) {
            return row * 100;
        }
        row += 1;
        for (x, c) in window[1].chars().enumerate() {
            columns_str[x].push(c);
        }
    }
    let columns: Vec<&str> = columns_str.iter().map(|s| s.as_str()).collect();
    let mut col = 1;
    for window in columns.windows(2) {
        if window[0] == window[1] && check_mirror(&columns, col) {
            return col;
        }
        col += 1;
    }
    panic!("invalid pattern");
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| find_reflection(pattern))
        .sum()
}

fn compare(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .filter(|&(c1, c2)| c1 != c2)
        .count()
}

fn check_mirror_with_smudge(lines: &Vec<&str>, split_at: usize) -> bool {
    let (first, second) = lines.split_at(split_at);
    let n = std::cmp::min(first.len(), second.len());
    let mirror_first: Vec<&str> = first.iter().rev().cloned().take(n).collect();
    let mirror_second: Vec<&str> = second.iter().cloned().take(n).collect();
    let mut smudges = 0;
    for i in 0..n {
        if smudges > 1 {
            return false;
        }
        let a = mirror_first[i];
        let b = mirror_second[i];
        smudges += compare(a, b);
    }
    smudges == 1
}

fn find_reflection2(pattern: &str) -> usize {
    let lines: Vec<&str> = pattern.lines().collect();
    let n = lines.len();
    let mut columns_str: Vec<String> = vec![String::new(); lines[0].len()];
    for (x, c) in lines[0].chars().enumerate() {
        columns_str[x].push(c);
    }
    for row in 1..n {
        if check_mirror_with_smudge(&lines, row) {
            return row * 100;
        }
        for (x, c) in lines[row].chars().enumerate() {
            columns_str[x].push(c);
        }
    }
    let columns: Vec<&str> = columns_str.iter().map(|s| s.as_str()).collect();
    for col in 1..columns.len() {
        if check_mirror_with_smudge(&columns, col) {
            return col;
        }
    }
    panic!("invalid pattern");
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| find_reflection2(pattern))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_reflection() {
        assert_eq!(
            find_reflection(
                "###.##.######
....##.......
##..##..####.
..#.##.......
..#....#....#
#........##..
###....######
..#....#....#
.#.####.#..#.
#.##..##.##.#
.#.####.#..#.
.########..##
#.##..##.##.#"
            ),
            10
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 405)
    }

    #[test]
    fn test_reflection2() {
        assert_eq!(
            find_reflection2(
                "###....####..#.##
##.#..#.#####.###
....##....###..##
...#..#....#.##..
.########..#.....
..######...######
.#..##..#..#.#..#"
            ),
            16
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 400)
    }
}
