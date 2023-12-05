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

#[derive(Debug, Eq, PartialEq)]
struct Mapping {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

fn parse_mapping(mapping_line: &str) -> Mapping {
    let numbers: Vec<u64> = mapping_line
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect();
    Mapping {
        dst_start: numbers[0],
        src_start: numbers[1],
        len: numbers[2],
    }
}

fn parse_map(map_lines: &str) -> Vec<Mapping> {
    map_lines
        .lines()
        .skip(1)
        .map(|mapping_line| parse_mapping(mapping_line))
        .collect()
}

fn part1_apply_map(map: &Vec<Mapping>, src: u64) -> u64 {
    for mapping in map {
        if src >= mapping.src_start && src < mapping.src_start + mapping.len {
            let offset = src - mapping.src_start;
            return mapping.dst_start + offset;
        }
    }
    src
}

fn part1_parse_seeds(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .skip(1) // Skip the 'seeds:' heading
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn part1(input: &str) -> u64 {
    let paragraphs: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<u64> = part1_parse_seeds(paragraphs[0]);
    let maps: Vec<Vec<Mapping>> = paragraphs
        .iter()
        .skip(1)
        .map(|map_lines| parse_map(map_lines))
        .collect();
    let mut lowest = u64::MAX;
    for seed in seeds {
        let mut num = seed;
        for map in &maps {
            num = part1_apply_map(&map, num);
        }
        if num < lowest {
            lowest = num;
        }
    }
    lowest
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

fn part2_parse_seeds(line: &str) -> Vec<Range> {
    let numbers = part1_parse_seeds(line);
    numbers
        .iter()
        .cloned()
        .step_by(2)
        .zip(numbers.iter().cloned().skip(1).step_by(2))
        .map(|(start, len)| Range {
            start,
            end: start + len - 1,
        })
        .collect()
}

fn ranges_overlap(a: &Range, b: &Range) -> bool {
    a.start <= b.end && b.start <= a.end
}

/**
 * Split a range according to mapping rule. If the rule does not apply return
 * None. Otherwise return a tuple (converted, leftovers) where converted is
 * the new destination range and leftovers are the parts that start or end
 * outside the mapping rule.
 */
fn split_range(range: &Range, mapping: &Mapping) -> Option<(Range, Vec<Range>)> {
    let src_range = Range {
        start: mapping.src_start,
        end: mapping.src_start + mapping.len - 1,
    };
    if !ranges_overlap(range, &src_range) {
        return None;
    } else {
        let mut leftovers = Vec::new();
        let mut offset = 0;
        let mut len = range.end - range.start;
        if range.start < src_range.start {
            let before = Range {
                start: range.start,
                end: src_range.start - 1,
            };
            leftovers.push(before);
            len -= src_range.start - range.start;
        } else {
            offset = range.start - src_range.start;
        }
        if range.end > src_range.end {
            let after = Range {
                start: src_range.end + 1,
                end: range.end,
            };
            leftovers.push(after);
            len -= range.end - src_range.end;
        }
        let middle = Range {
            start: mapping.dst_start + offset,
            end: mapping.dst_start + offset + len,
        };
        Some((middle, leftovers))
    }
}

// NOTE: The src ranges are processed in reverse.
fn part2_apply_map(map: &Vec<Mapping>, src: &Vec<Range>) -> Vec<Range> {
    let mut results: Vec<Range> = Vec::new();
    let mut remaining: Vec<Range> = src.clone();
    while remaining.len() > 0 {
        let range = remaining.pop().unwrap();
        let mut found = false;
        for mapping in map {
            if let Some((converted, mut leftovers)) = split_range(&range, mapping) {
                results.push(converted);
                remaining.append(&mut leftovers);
                found = true;
                break;
            }
        }
        if !found {
            results.push(range);
        }
    }
    results
}

fn part2(input: &str) -> u64 {
    let paragraphs: Vec<&str> = input.split("\n\n").collect();
    let mut ranges: Vec<Range> = part2_parse_seeds(paragraphs[0]);
    let maps: Vec<Vec<Mapping>> = paragraphs
        .iter()
        .skip(1)
        .map(|map_lines| parse_map(map_lines))
        .collect();
    for map in &maps {
        ranges = part2_apply_map(map, &ranges);
    }
    let mut lowest = u64::MAX;
    for range in &ranges {
        if range.start < lowest {
            lowest = range.start;
        }
    }
    lowest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_part1_seeds() {
        assert_eq!(
            part1_parse_seeds("seeds: 79 14 55 13"),
            vec!(79, 14, 55, 13)
        )
    }

    #[test]
    fn test_parse_mapping() {
        let expected = Mapping {
            dst_start: 50,
            src_start: 98,
            len: 2,
        };
        assert_eq!(parse_mapping("50 98 2"), expected)
    }

    #[test]
    fn test_parse_map() {
        let expected = vec![
            Mapping {
                dst_start: 50,
                src_start: 98,
                len: 2,
            },
            Mapping {
                dst_start: 52,
                src_start: 50,
                len: 48,
            },
        ];
        assert_eq!(
            parse_map(
                "seed-to-soil map:
50 98 2
52 50 48"
            ),
            expected
        )
    }

    #[test]
    fn test_part1_apply_map() {
        let map = vec![
            Mapping {
                dst_start: 50,
                src_start: 98,
                len: 2,
            },
            Mapping {
                dst_start: 52,
                src_start: 50,
                len: 48,
            },
        ];
        assert_eq!(part1_apply_map(&map, 98), 50);
        assert_eq!(part1_apply_map(&map, 99), 51);
        assert_eq!(part1_apply_map(&map, 100), 100);
        assert_eq!(part1_apply_map(&map, 50), 52);
        assert_eq!(part1_apply_map(&map, 51), 53);
        assert_eq!(part1_apply_map(&map, 50 + 47), 52 + 47);
    }

    static SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 35)
    }

    #[test]
    fn test_part2_parse_seeds() {
        assert_eq!(
            part2_parse_seeds("seeds: 79 14 55 13"),
            vec!(Range { start: 79, end: 92 }, Range { start: 55, end: 67 })
        );
    }

    #[test]
    fn test_ranges_overlap() {
        let range1 = Range { start: 10, end: 15 };
        let range2 = Range { start: 12, end: 20 };
        let range3 = Range { start: 20, end: 25 };
        // test when end between start and end.
        // range1 10 -> 15
        // range2    12 -> 20
        assert!(ranges_overlap(&range1, &range2));
        assert!(ranges_overlap(&range2, &range1));
        // test when share start/end.
        // range2 12 -> 20
        // range3       20 -> 25
        assert!(ranges_overlap(&range2, &range3));
        assert!(ranges_overlap(&range3, &range2));
        // test range before/after another.
        assert!(!ranges_overlap(&range1, &range3));
        assert!(!ranges_overlap(&range3, &range1));
        // test range contained by another.
        // range4 5    ->    20
        // range1   10 -> 15
        let range4 = Range { start: 5, end: 20 };
        assert!(ranges_overlap(&range1, &range4));
        assert!(ranges_overlap(&range4, &range1));
    }

    #[test]
    fn test_split_range() {
        let range1 = Range { start: 10, end: 25 };
        let mapping1 = Mapping {
            dst_start: 50,
            src_start: 15,
            len: 5,
        };
        assert_eq!(
            split_range(&range1, &mapping1),
            Some((
                Range { start: 50, end: 54 },
                vec!(Range { start: 10, end: 14 }, Range { start: 20, end: 25 },)
            ))
        )
    }

    #[test]
    fn test_part2_apply_map() {
        let map = vec![
            Mapping {
                dst_start: 50,
                src_start: 98,
                len: 2,
            },
            Mapping {
                dst_start: 52,
                src_start: 50,
                len: 48,
            },
        ];
        let input = vec![Range { start: 79, end: 92 }, Range { start: 55, end: 67 }];
        assert_eq!(
            part2_apply_map(&map, &input),
            vec!(Range { start: 57, end: 69 }, Range { start: 81, end: 94 },)
        )
    }

    #[test]
    fn test_part2_apply_map_multiple() {
        // A range can have multiple different mappings applied to it.
        // This use-case had me stumped for ages, as the code worked on the
        // sample in the puzzle, but not on the input. The initial version of
        // apply_map only allowed 1 mapping per Range.
        let map = vec![
            Mapping {
                dst_start: 50,
                src_start: 98,
                len: 2,
            },
            Mapping {
                dst_start: 52,
                src_start: 50,
                len: 48,
            },
        ];
        let input = vec![Range { start: 1, end: 200 }];
        assert_eq!(
            part2_apply_map(&map, &input),
            vec!(
                // mapping of 50 98 2
                // Range 98:99 -> 50:51
                Range { start: 50, end: 51 },
                // no mapping for >= 100
                Range {
                    start: 100,
                    end: 200
                },
                // mapping of 52 50 48
                // Range 50:97 -> 52:99
                Range { start: 52, end: 99 },
                // no mapping for < 50
                Range { start: 1, end: 49 }
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 46)
    }
}
