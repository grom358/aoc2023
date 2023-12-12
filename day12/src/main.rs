use memoize::memoize;
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Row {
    conditions: Vec<Condition>,
    group_sizes: Vec<usize>,
}

impl ToString for Row {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for c in &self.conditions {
            match c {
                Condition::Damaged => result.push('#'),
                Condition::Operational => result.push('.'),
                Condition::Unknown => result.push('?'),
            }
        }
        result.push(' ');
        for (idx, group_size) in self.group_sizes.iter().enumerate() {
            if idx > 0 {
                result.push(',');
            }
            result.push_str(group_size.to_string().as_str());
        }
        result
    }
}

fn parse_line(line: &str) -> Row {
    let parts: Vec<&str> = line.split(' ').collect();
    let conditions: Vec<Condition> = parts[0]
        .chars()
        .map(|c| match c {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Invalid condition"),
        })
        .collect();
    let group_sizes: Vec<usize> = parts[1]
        .split(',')
        .map(|num_str| num_str.parse().unwrap())
        .collect();
    Row {
        conditions,
        group_sizes,
    }
}

/**
 * The possible combinations after match first group.
 */
fn after_group_combinations(conditions: Vec<Condition>, group_sizes: Vec<usize>) -> usize {
    let group_size = group_sizes[0];
    if conditions.len() >= group_size
        && conditions[..group_size]
            .iter()
            .all(|c| *c == Condition::Damaged || *c == Condition::Unknown)
    {
        let new_conditions = conditions[group_size..].to_vec();
        let new_group_sizes = group_sizes[1..].to_vec();
        if new_conditions.len() > 0 {
            if new_conditions[0] == Condition::Operational
                || new_conditions[0] == Condition::Unknown
            {
                combinations(new_conditions[1..].to_vec(), new_group_sizes)
            } else {
                0
            }
        } else {
            combinations(new_conditions, new_group_sizes)
        }
    } else {
        0
    }
}

#[memoize]
fn combinations(conditions: Vec<Condition>, group_sizes: Vec<usize>) -> usize {
    if group_sizes.len() == 0 {
        if conditions.len() == 0 {
            // Valid combination: all groups processed with no remaining input.
            1
        } else {
            if conditions
                .iter()
                .all(|c| *c == Condition::Operational || *c == Condition::Unknown)
            {
                // Valid combination: all groups processed so all remaining
                // Unknowns are Operational.
                1
            } else {
                // Invalid combination: No groups left but there are still
                // Damaged groups.
                0
            }
        }
    } else if conditions.len() == 0 {
        // Invalid combination: There no input left but still have remaining
        // Damaged groups.
        0
    } else {
        match conditions[0] {
            Condition::Operational => {
                let new_conditions = conditions
                    .iter()
                    .skip_while(|c| **c == Condition::Operational)
                    .map(|c| *c)
                    .collect();
                combinations(new_conditions, group_sizes)
            }
            Condition::Damaged => after_group_combinations(conditions, group_sizes),
            Condition::Unknown => {
                let new_conditions = conditions
                    .iter()
                    .skip(1) // skip Unknown
                    .skip_while(|c| **c == Condition::Operational)
                    .map(|c| *c)
                    .collect();
                // sum of both when Unknown is Operational and Damaged
                combinations(new_conditions, group_sizes.clone())
                    + after_group_combinations(conditions, group_sizes)
            }
        }
    }
}

fn total_combinations(rows: Vec<Row>) -> usize {
    let mut total = 0;
    for row in rows {
        total += combinations(row.conditions, row.group_sizes);
    }
    total
}

fn part1(input: &str) -> usize {
    let rows: Vec<Row> = input.lines().map(|line| parse_line(line)).collect();
    total_combinations(rows)
}

fn unfold(folded: &Row) -> Row {
    Row {
        conditions: (0..4)
            .flat_map(|_| {
                folded
                    .conditions
                    .iter()
                    .cloned()
                    .chain(vec![Condition::Unknown])
            })
            .chain(folded.conditions.clone())
            .collect(),
        group_sizes: (0..5).flat_map(|_| folded.group_sizes.clone()).collect(),
    }
}

fn part2(input: &str) -> usize {
    let rows: Vec<Row> = input
        .lines()
        .map(|line| unfold(&parse_line(line)))
        .collect();
    total_combinations(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 21)
    }

    #[test]
    fn test_unfold() {
        let row = parse_line("???.### 1,1,3");
        assert_eq!(
            unfold(&row).to_string().as_str(),
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 525152)
    }
}
