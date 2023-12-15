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

fn hash(s: &str) -> usize {
    let mut value: usize = 0;
    for c in s.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }
    value
}

fn part1(input: &str) -> usize {
    input.trim().split(',').map(|s| hash(s)).sum()
}

fn part2(input: &str) -> usize {
    let operations: Vec<&str> = input.trim().split(',').collect();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    for op in operations {
        if op.ends_with("-") {
            let label = &op[..op.len() - 1];
            let box_idx = hash(label);
            if let Some(index) = boxes[box_idx]
                .iter()
                .position(|&(len_label, _)| len_label == label)
            {
                boxes[box_idx].remove(index);
            }
        } else {
            let parts: Vec<&str> = op.split('=').collect();
            let label = parts[0];
            let focal_len: usize = parts[1].parse().unwrap();
            let box_idx = hash(label);
            if let Some(index) = boxes[box_idx]
                .iter()
                .position(|&(len_label, _)| len_label == label)
            {
                boxes[box_idx][index] = (label, focal_len);
            } else {
                boxes[box_idx].push((label, focal_len));
            }
        }
    }
    let mut focusing_power = 0;
    for (box_idx, b) in boxes.iter().enumerate() {
        for (slot_idx, (_, focal_len)) in b.iter().enumerate() {
            focusing_power += (box_idx + 1) * (slot_idx + 1) * focal_len;
        }
    }
    focusing_power
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1320)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 145)
    }
}
