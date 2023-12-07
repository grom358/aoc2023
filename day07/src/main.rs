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

fn part1_card_value(c: char) -> u8 {
    if let Some(x) = c.to_digit(10) {
        x as u8
    } else {
        match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid input!"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    hand_strength: u8,
    cards: Vec<u8>,
    bid: usize,
}

const HIGH_CARD: u8 = 0;
const ONE_PAIR: u8 = 1;
const TWO_PAIR: u8 = 2;
const THREE_OF_A_KIND: u8 = 3;
const FULLHOUSE: u8 = 4;
const FOUR_OF_A_KIND: u8 = 5;
const FIVE_OF_A_KIND: u8 = 6;

struct CardCount {
    card: u8,
    count: u8,
}

fn part1_parse_hand(str_hand: &str, bid: usize) -> Hand {
    let cards: Vec<u8> = str_hand.chars().map(|c| part1_card_value(c)).collect();
    let mut card_counts: Vec<CardCount> = Vec::new();
    for card in &cards {
        let mut found = false;
        for cc in card_counts.iter_mut() {
            if cc.card == *card {
                cc.count += 1;
                found = true;
            }
        }
        if !found {
            card_counts.push(CardCount {
                card: *card,
                count: 1,
            });
        }
    }
    let hand_strength = if card_counts[0].count == 5 {
        FIVE_OF_A_KIND
    } else if card_counts[0].count == 4 || card_counts[1].count == 4 {
        FOUR_OF_A_KIND
    } else if card_counts.iter().filter(|cc| cc.count == 3).count() == 1 {
        if card_counts.len() == 2 {
            FULLHOUSE
        } else {
            THREE_OF_A_KIND
        }
    } else {
        let pairs = card_counts.iter().filter(|cc| cc.count == 2).count();
        if pairs == 2 {
            TWO_PAIR
        } else if pairs == 1 {
            ONE_PAIR
        } else {
            HIGH_CARD
        }
    };
    Hand {
        hand_strength,
        cards,
        bid,
    }
}

fn parse_line(line: &str, parse_hand: fn(&str, usize) -> Hand) -> Hand {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let bid: usize = parts[1].parse().unwrap();
    parse_hand(parts[0], bid)
}

fn parse(input: &str, parse_hand: fn(&str, usize) -> Hand) -> Vec<Hand> {
    input
        .lines()
        .map(|line| parse_line(line, parse_hand))
        .collect()
}

fn calculate_winnings(input: &str, parse_hand: fn(&str, usize) -> Hand) -> usize {
    let mut hands = parse(input, parse_hand);
    hands.sort();
    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index + 1) * hand.bid;
    }
    total
}

fn part1(input: &str) -> usize {
    calculate_winnings(input, part1_parse_hand)
}

fn part2_card_value(c: char) -> u8 {
    if c == 'J' {
        1
    } else {
        part1_card_value(c)
    }
}

fn part2_parse_hand(str_hand: &str, bid: usize) -> Hand {
    let cards: Vec<u8> = str_hand.chars().map(|c| part2_card_value(c)).collect();
    let mut card_counts: Vec<CardCount> = Vec::new();
    let mut jokers = 0;
    for card in &cards {
        if *card == 1 {
            jokers += 1;
        } else {
            let mut found = false;
            for cc in card_counts.iter_mut() {
                if cc.card == *card {
                    cc.count += 1;
                    found = true;
                }
            }
            if !found {
                card_counts.push(CardCount {
                    card: *card,
                    count: 1,
                });
            }
        }
    }
    let has_four_kind = card_counts.iter().filter(|cc| cc.count == 4).count() == 1;
    let has_three_kind = card_counts.iter().filter(|cc| cc.count == 3).count() == 1;
    let pairs = card_counts.iter().filter(|cc| cc.count == 2).count();
    let hand_strength = if (card_counts.len() == 1 && card_counts[0].count == 5)
        || (has_four_kind && jokers == 1)
        || (has_three_kind && jokers == 2)
        || (pairs == 1 && jokers == 3)
        || jokers >= 4
    {
        FIVE_OF_A_KIND
    } else if has_four_kind
        || (has_three_kind && jokers == 1)
        || (pairs == 1 && jokers == 2)
        || jokers == 3
    {
        FOUR_OF_A_KIND
    } else if (has_three_kind && pairs == 1)
        || (has_three_kind && jokers == 1)
        || (pairs == 2 && jokers == 1)
        || (pairs == 1 && jokers == 2)
    {
        FULLHOUSE
    } else if has_three_kind || (pairs == 1 && jokers == 1) || jokers == 2 {
        THREE_OF_A_KIND
    } else if pairs == 2 || (pairs == 1 && jokers == 1) {
        TWO_PAIR
    } else if pairs == 1 || jokers == 1 {
        ONE_PAIR
    } else {
        HIGH_CARD
    };
    let results = Hand {
        hand_strength,
        cards,
        bid,
    };
    results
}

fn part2(input: &str) -> usize {
    calculate_winnings(input, part2_parse_hand)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 6440)
    }

    #[test]
    fn test_parse_part2() {
        assert_eq!(
            part2_parse_hand("32T3K", 765),
            Hand {
                hand_strength: ONE_PAIR,
                cards: vec!(3, 2, 10, 3, 13),
                bid: 765,
            }
        );
        assert_eq!(
            part2_parse_hand("T55J5", 684),
            Hand {
                hand_strength: FOUR_OF_A_KIND,
                cards: vec!(10, 5, 5, 1, 5),
                bid: 684,
            }
        );
        assert_eq!(
            part2_parse_hand("KK677", 28),
            Hand {
                hand_strength: TWO_PAIR,
                cards: vec!(13, 13, 6, 7, 7),
                bid: 28,
            }
        );
        assert_eq!(
            part2_parse_hand("KTJJT", 220),
            Hand {
                hand_strength: FOUR_OF_A_KIND,
                cards: vec!(13, 10, 1, 1, 10),
                bid: 220,
            }
        );
        assert_eq!(
            part2_parse_hand("QQQJA", 483),
            Hand {
                hand_strength: FOUR_OF_A_KIND,
                cards: vec!(12, 12, 12, 1, 14),
                bid: 483,
            }
        );
        assert_eq!(
            part2_parse_hand("4JJKQ", 84),
            Hand {
                hand_strength: THREE_OF_A_KIND,
                cards: vec!(4, 1, 1, 13, 12),
                bid: 84,
            }
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 5905)
    }
}
