use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)?;
    let input = file_contents.as_str();

    println!("Part 1: {}", part1(input));

    Ok(())
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let key = parts[0];
        let connected_to: Vec<&str> = parts[1].split_whitespace().collect();
        for node in connected_to {
            // Connect in both directions.
            graph.entry(key).or_default().insert(node);
            graph.entry(node).or_default().insert(key);
        }
    }
    graph
}

// Find the link that is used the most in reaching other nodes.
fn find_link<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> (&'a str, &'a str) {
    let mut links: HashMap<(&str, &str), usize> = HashMap::new();
    for start in graph.keys().cloned() {
        // Use bfs to find connected nodes.
        let mut queue = VecDeque::from([start]);
        let mut seen = HashSet::from([start]);
        while let Some(curr) = queue.pop_front() {
            for &node in graph[&curr].iter() {
                if !seen.insert(node) {
                    continue;
                }
                queue.push_back(node);
                let link = if curr < node {
                    (curr, node)
                } else {
                    (node, curr)
                };
                *links.entry(link).or_default() += 1;
            }
        }
    }
    links
        .into_iter()
        .max_by_key(|(_k, v)| *v)
        .map(|(k, _v)| k)
        .unwrap()
}

// Use bfs to find the number of reachable nodes in the graph.
fn reachable_size(graph: &HashMap<&str, HashSet<&str>>) -> usize {
    let start = graph.keys().next().cloned().unwrap();
    let mut queue = VecDeque::from([start]);
    let mut seen = HashSet::new();
    while let Some(curr) = queue.pop_front() {
        if seen.insert(curr) {
            queue.extend(graph[&curr].iter().cloned())
        }
    }
    seen.len()
}

fn part1(input: &str) -> usize {
    let mut graph = parse(input);
    // Make 3 cuts.
    for _ in 0..3 {
        let (left, right) = find_link(&graph);
        graph.get_mut(&left).map(|v| v.remove(&right));
        graph.get_mut(&right).map(|v| v.remove(&left));
    }
    let size = reachable_size(&graph);
    size * (graph.len() - size)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 54)
    }
}
