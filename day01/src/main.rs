use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    let mut p1_total = 0;
    let mut p2_total = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            p1_total += parse_line_p1(&line);
            p2_total += parse_line_p2(&line);
        }
    }
    println!("Part 1: {}", p1_total);
    println!("Part 2: {}", p2_total);
    Ok(())
}

pub fn parse_line_p1(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() == 0 {
        return 0;
    }
    digits[0] * 10 + digits[digits.len() - 1]
}

fn parse_digit(s: &str) -> Option<u32> {
    static WORDS: [(&str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    s.chars().next().and_then(|first_char| {
        if first_char.is_digit(10) {
            first_char.to_digit(10)
        } else {
            for (word, value) in WORDS.iter() {
                if s.starts_with(word) {
                    return Some(*value);
                }
            }
            None
        }
    })
}

fn first_digit(line: &str) -> u32 {
    for (i, _) in line.char_indices() {
        let substring = &line[i..];
        if let Some(digit) = parse_digit(substring) {
            return digit;
        }
    }
    0
}

fn last_digit(line: &str) -> u32 {
    for i in (0..=line.len()).rev() {
        let substring = &line[i..];
        if let Some(digit) = parse_digit(substring) {
            return digit;
        }
    }
    0
}

pub fn parse_line_p2(line: &str) -> u32 {
    let first = first_digit(line);
    let last = last_digit(line);
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(parse_line_p1("1abc2"), 12);
        assert_eq!(parse_line_p1("pqr3stu8vwx"), 38);
        assert_eq!(parse_line_p1("a1b2c3d4e5f"), 15);
        assert_eq!(parse_line_p1("treb7uchet"), 77);
    }

    #[test]
    fn test_part2() {
        assert_eq!(parse_line_p2("two1nine"), 29);
        assert_eq!(parse_line_p2("eightwothree"), 83);
        assert_eq!(parse_line_p2("abcone2threexyz"), 13);
        assert_eq!(parse_line_p2("xtwone3four"), 24);
        assert_eq!(parse_line_p2("4nineeightseven2"), 42);
        assert_eq!(parse_line_p2("zoneight234"), 14);
        assert_eq!(parse_line_p2("7pqrstsixteen"), 76);
    }
}
