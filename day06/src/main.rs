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

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

fn part1_parse_line(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .skip(1)
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn part1_parse(input: &str) -> Vec<Race> {
    let mut results = Vec::new();
    let numbers: Vec<Vec<u64>> = input.lines().map(|line| part1_parse_line(line)).collect();
    let times = &numbers[0];
    let records = &numbers[1];
    for i in 0..times.len() {
        results.push(Race {
            time: times[i],
            record: records[i],
        });
    }
    results
}

fn part1(input: &str) -> u64 {
    let races = part1_parse(input);
    let mut counts = Vec::with_capacity(races.len());
    for race in races {
        let mut count = 0;
        for i in 1..race.time {
            let distance = i * (race.time - i);
            if distance > race.record {
                count += 1;
            }
        }
        counts.push(count);
    }
    counts.iter().product()
}

fn part2_parse_line(line: &str) -> u64 {
    let str_value: String = line
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect();
    str_value.parse().unwrap()
}

fn part2_parse(input: &str) -> Race {
    let numbers: Vec<u64> = input.lines().map(|line| part2_parse_line(line)).collect();
    Race {
        time: numbers[0],
        record: numbers[1],
    }
}

fn quadractic_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let sqrt_discriminant = discriminant.sqrt();
    let root1 = (-b + sqrt_discriminant) / (2.0 * a);
    let root2 = (-b - sqrt_discriminant) / (2.0 * a);

    let (min_root, max_root) = if root1 < root2 {
        (root1, root2)
    } else {
        (root2, root1)
    };
    Some((min_root, max_root))
}

fn part2(input: &str) -> u64 {
    let race = part2_parse(input);
    let (min_bound, max_bound) =
        quadractic_roots(-1.0, race.time as f64, -1.0 * race.record as f64).unwrap();
    (max_bound as u64) - (min_bound as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 288)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 71503)
    }
}
