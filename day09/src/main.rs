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

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn extrapolate(numbers: &Vec<i32>) -> i32 {
    let mut input = numbers.clone();
    let mut history: Vec<i32> = Vec::new();
    loop {
        let mut differences: Vec<i32> = Vec::with_capacity(input.len() - 1);
        for (index, &value) in input.iter().enumerate().skip(1) {
            let previous = input[index - 1];
            differences.push(value - previous);
        }
        history.push(*input.last().unwrap());
        if differences.iter().all(|&x| x == 0) {
            break;
        }
        input = differences;
    }
    history.reverse();
    let mut diff = 0;
    for value in history {
        diff += value;
    }
    diff
}

fn part1(input: &str) -> i32 {
    input.lines()
        .map(|line| extrapolate(&parse_line(line)))
        .sum()
}

fn extrapolate_backwards(numbers: &Vec<i32>) -> i32 {
    let mut input = numbers.clone();
    let mut history: Vec<i32> = Vec::new();
    loop {
        let mut differences: Vec<i32> = Vec::with_capacity(input.len() - 1);
        for (index, &value) in input.iter().enumerate().skip(1) {
            let previous = input[index - 1];
            differences.push(previous - value);
        }
        history.push(*input.first().unwrap());
        if differences.iter().all(|&x| x == 0) {
            break;
        }
        input = differences;
    }
    history.reverse();
    let mut diff = 0;
    for value in history {
        diff += value;
    }
    diff
}

fn part2(input: &str) -> i32 {
    input.lines()
        .map(|line| extrapolate_backwards(&parse_line(line)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&vec!(0, 3, 6, 9, 12, 15)), 18);
        assert_eq!(extrapolate(&vec!(1, 3, 6, 10, 15, 21)), 28);
        assert_eq!(extrapolate(&vec!(10, 13, 16, 21, 30, 45)), 68);
        assert_eq!(extrapolate(&vec!(14, 16, 26, 50, 86, 127, 182, 334, 860, 2447, 6555, 16007, 35930, 75232, 148879, 281333, 511626, 900674, 1541576, 2573792, 4202246)), 6722549);
        assert_eq!(extrapolate(&vec!(1, 12, 32, 60, 95, 136, 182, 232, 285, 340, 396, 452, 507, 560, 610, 656, 697, 732, 760, 780, 791)), 792)
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"),
            114
        )
    }

    #[test]
    fn test_extrapolate_backwards() {
        assert_eq!(extrapolate_backwards(&vec!(0, 3, 6, 9, 12, 15)), -3);
        assert_eq!(extrapolate_backwards(&vec!(1, 3, 6, 10, 15, 21)), 0);
        assert_eq!(extrapolate_backwards(&vec!(10, 13, 16, 21, 30, 45)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"),
            2
        )
    }
}
