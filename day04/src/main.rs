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

fn process_numbers(s: &str) -> Vec<u32> {
    let str_numbers = s.trim().split_whitespace();
    let mut numbers = Vec::new();
    for str_num in str_numbers {
        let number = str_num.parse::<u32>().unwrap();
        numbers.push(number);
    }
    numbers
}

fn process_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let top_level_parts: Vec<&str> = line.split(':').collect();
    let card_part = top_level_parts[1];
    let card_parts: Vec<&str> = card_part.split('|').collect();
    let winning_numbers = process_numbers(card_parts[0]);
    let numbers = process_numbers(card_parts[1]);
    (winning_numbers, numbers)
}

fn process_lines(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input.lines().map(|line| process_line(line)).collect()
}

fn part1(input: &str) -> u32 {
    let cards = process_lines(input);
    let mut total = 0;
    for (winning_numbers, numbers) in cards {
        let mut points = 0;
        for num in &numbers {
            // Number of winning numbers is not that large, just linear search.
            for winning_number in &winning_numbers {
                if num == winning_number {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                    break;
                }
            }
        }
        total += points;
    }
    total
}

fn part2(input: &str) -> u32 {
    let cards = process_lines(input);
    let mut copies = vec![1; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let multipler = copies[index];
        let (winning_numbers, numbers) = card;
        let mut matches = 0;
        for num in numbers {
            for winning_number in winning_numbers {
                if num == winning_number {
                    matches += 1;
                    break;
                }
            }
        }
        for offset in 1..=matches {
            copies[index + offset] += multipler;
        }
    }
    copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 30)
    }
}
