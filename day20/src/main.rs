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
    println!("Part 2: {}", part2(input));

    Ok(())
}

#[derive(Debug)]
enum Module<'a> {
    Broadcaster {
        outputs: Vec<&'a str>,
    },
    FlipFlop {
        state: bool,
        outputs: Vec<&'a str>,
    },
    Conjuction {
        states: HashMap<&'a str, bool>,
        outputs: Vec<&'a str>,
    },
}

fn parse_line(line: &str) -> (&str, Module) {
    let parts: Vec<&str> = line.split(" -> ").collect();
    let outputs: Vec<&str> = parts[1].split(", ").collect();
    if parts[0] == "broadcaster" {
        ("broadcaster", Module::Broadcaster { outputs })
    } else {
        let prefix = parts[0].chars().next().unwrap();
        let label = &parts[0][1..];
        match prefix {
            '%' => (
                label,
                Module::FlipFlop {
                    state: false,
                    outputs,
                },
            ),
            '&' => (
                label,
                Module::Conjuction {
                    states: HashMap::new(),
                    outputs,
                },
            ),
            _ => panic!("invalid module"),
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Module> {
    let mut modules: HashMap<&str, Module> = HashMap::new();
    let mut conjuctions: HashSet<&str> = HashSet::new();
    for (label, module) in input.lines().map(|line| parse_line(line)) {
        match module {
            Module::Conjuction {
                states: _,
                outputs: _,
            } => {
                conjuctions.insert(label);
            }
            _ => (),
        }
        modules.insert(label, module);
    }
    // Initialize conjuctions.
    let mut conjunction_links: Vec<(&str, &str)> = Vec::new();
    for (from, module) in modules.iter() {
        let outputs = match module {
            Module::Conjuction { states: _, outputs } => outputs,
            Module::FlipFlop { state: _, outputs } => outputs,
            Module::Broadcaster { outputs } => outputs,
        };
        for to in outputs {
            if conjuctions.contains(to) {
                conjunction_links.push((from, to));
            }
        }
    }
    for (from, to) in conjunction_links {
        if let Module::Conjuction { states, outputs: _ } = modules.get_mut(to).unwrap() {
            states.insert(from, false);
        }
    }
    modules
}

#[derive(Debug)]
struct Monitor<'a> {
    presses: usize,
    rx_conjunctor: &'a str,
    periods: Vec<usize>,
}

fn press_button(
    modules: &mut HashMap<&str, Module>,
    part2: Option<&mut Monitor>,
) -> (usize, usize) {
    let mut low_pulses = 1; // Button sends low pulse to broadcaster.
    let mut high_pulses = 0;
    let mut signals: VecDeque<(&str, &str, bool)> = VecDeque::new();
    if let Module::Broadcaster {
        outputs: initial_outputs,
    } = modules.get("broadcaster").unwrap()
    {
        let from = "broadcaster";
        for to in initial_outputs {
            signals.push_back((from, *to, false));
        }
    } else {
        unreachable!()
    }
    while signals.len() > 0 {
        let (from, to, pulse) = signals.pop_front().unwrap();
        if pulse {
            high_pulses += 1;
        } else {
            low_pulses += 1;
        }
        if !modules.contains_key(to) {
            continue;
        }
        let dst_module = modules.get_mut(to).unwrap();
        match dst_module {
            Module::FlipFlop { state, outputs } => {
                if !pulse {
                    *state = !*state;
                    for label in outputs {
                        signals.push_back((to, label, *state));
                    }
                }
            }
            Module::Conjuction { states, outputs } => {
                if pulse {
                    if let Some(Monitor {
                        presses,
                        rx_conjunctor,
                        periods,
                    }) = part2
                    {
                        if to == *rx_conjunctor {
                            periods.push(*presses);
                        }
                    }
                }
                states.insert(from, pulse);
                let conjuction_pulse = !states.values().all(|s| *s);
                for label in outputs {
                    signals.push_back((to, label, conjuction_pulse));
                }
            }
            _ => unreachable!(),
        }
    }
    (low_pulses, high_pulses)
}

fn part1(input: &str) -> usize {
    let mut modules = parse(input);
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let (lo, hi) = press_button(&mut modules, None);
        low_pulses += lo;
        high_pulses += hi;
    }
    low_pulses * high_pulses
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(numbers: Vec<usize>) -> usize {
    numbers
        .iter()
        .fold(1, |lcm, &num| lcm * num / gcd(lcm, num))
}

fn find_rx_conjunctor<'a>(modules: &HashMap<&'a str, Module>) -> &'a str {
    for (from, module) in modules.iter() {
        let outputs = match module {
            Module::Broadcaster { outputs } => outputs,
            Module::FlipFlop { state: _, outputs } => outputs,
            Module::Conjuction { states: _, outputs } => outputs,
        };
        for to in outputs {
            if *to == "rx" {
                return from;
            }
        }
    }
    panic!("No rx found");
}

fn part2(input: &str) -> usize {
    let mut modules = parse(input);
    let rx_conjunctor = find_rx_conjunctor(&modules);
    let states = if let Some(Module::Conjuction { states, outputs: _ }) = modules.get(rx_conjunctor)
    {
        states
    } else {
        panic!("No rx conjunctor found!")
    };
    let rx_input_len = states.len();
    let mut monitor = Monitor {
        presses: 0,
        rx_conjunctor,
        periods: Vec::new(),
    };
    loop {
        monitor.presses += 1;
        press_button(&mut modules, Some(&mut monitor));
        if monitor.periods.len() >= rx_input_len {
            break;
        }
    }
    lcm(monitor.periods)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
            ),
            32000000
        );
        assert_eq!(
            part1(
                "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
            ),
            11687500
        )
    }
}
