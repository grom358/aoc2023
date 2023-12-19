use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'x' => Category::Cool,
            'm' => Category::Musical,
            'a' => Category::Aerodynamic,
            's' => Category::Shiny,
            _ => panic!("invalid category {c}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action<'a> {
    Goto(&'a str),
    Reject,
    Accept,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule<'a> {
    GreaterThan(Category, u64, Action<'a>),
    LessThan(Category, u64, Action<'a>),
    Otherwise(Action<'a>),
}

fn parse_label(label: &str) -> Action {
    match label {
        "A" => Action::Accept,
        "R" => Action::Reject,
        l => Action::Goto(l),
    }
}

fn parse_rule(rule: &str) -> Rule {
    if rule.contains(":") {
        let parts: Vec<&str> = rule.split(':').collect();
        let goto = parse_label(parts[1]);
        let c = parts[0].chars().next().unwrap();
        let category = Category::from(c);
        let is_greater = parts[0].contains(">");
        let value: u64 = if is_greater {
            parts[0].split('>').last().unwrap()
        } else {
            parts[0].split('<').last().unwrap()
        }
        .parse()
        .unwrap();
        if is_greater {
            Rule::GreaterThan(category, value, goto)
        } else {
            Rule::LessThan(category, value, goto)
        }
    } else {
        Rule::Otherwise(parse_label(rule))
    }
}

fn parse_workflow(line: &str) -> (&str, Vec<Rule>) {
    let index = line.find('{').unwrap();
    let label = &line[..index];
    let rules_str = &line[index + 1..line.len() - 1];
    let rules: Vec<Rule> = rules_str.split(',').map(|rule| parse_rule(rule)).collect();
    (label, rules)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rating {
    cool: u64,
    musical: u64,
    aerodynamic: u64,
    shiny: u64,
}

fn parse_rating(line: &str) -> Rating {
    let values: Vec<u64> = line
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .map(|p| p.split('=').last().unwrap().parse().unwrap())
        .collect();
    Rating {
        cool: values[0],
        musical: values[1],
        aerodynamic: values[2],
        shiny: values[3],
    }
}

fn parse(input: &str) -> (HashMap<&str, Vec<Rule>>, Vec<Rating>) {
    let mut workflows: HashMap<&str, Vec<Rule>> = HashMap::new();
    let parts: Vec<&str> = input.split("\n\n").collect();
    for line in parts[0].lines() {
        let (label, rules) = parse_workflow(line);
        workflows.insert(label, rules);
    }
    let ratings: Vec<Rating> = parts[1].lines().map(|line| parse_rating(line)).collect();
    (workflows, ratings)
}

fn rating_category_value(rating: &Rating, category: &Category) -> u64 {
    match category {
        Category::Cool => rating.cool,
        Category::Musical => rating.musical,
        Category::Aerodynamic => rating.aerodynamic,
        Category::Shiny => rating.shiny,
    }
}

fn rule_matches<'a>(rule: &'a Rule<'a>, rating: &'a Rating) -> Option<Action<'a>> {
    match rule {
        Rule::GreaterThan(category, value, action) => {
            let rating_value = rating_category_value(rating, category);
            if rating_value > *value {
                Some(*action)
            } else {
                None
            }
        }
        Rule::LessThan(category, value, action) => {
            let rating_value = rating_category_value(rating, category);
            if rating_value < *value {
                Some(*action)
            } else {
                None
            }
        }
        Rule::Otherwise(action) => Some(*action),
    }
}

fn get_action<'a>(rules: &'a Vec<Rule<'a>>, rating: &'a Rating) -> Action<'a> {
    for rule in rules {
        if let Some(action) = rule_matches(rule, rating) {
            return action;
        }
    }
    unreachable!()
}

fn part1(input: &str) -> u64 {
    let (workflow_map, ratings) = parse(input);
    let mut total: u64 = 0;
    for rating in ratings {
        let mut current = "in";
        loop {
            let rules = workflow_map.get(current).unwrap();
            let action = get_action(&rules, &rating);
            match action {
                Action::Accept => {
                    total +=
                        (rating.cool + rating.musical + rating.aerodynamic + rating.shiny) as u64;
                    break;
                }
                Action::Reject => break,
                Action::Goto(label) => current = label,
            }
        }
    }
    total
}

fn apply_bound(bound: Rating, category: &Category, value: u64) -> Rating {
    let mut new_bound = bound.clone();
    match category {
        Category::Cool => new_bound.cool = value,
        Category::Musical => new_bound.musical = value,
        Category::Aerodynamic => new_bound.aerodynamic = value,
        Category::Shiny => new_bound.shiny = value,
    }
    new_bound
}

fn rule_apply(rule: &Rule, lower_bound: Rating, upper_bound: Rating) -> (Rating, Rating) {
    match rule {
        Rule::GreaterThan(category, value, _) => {
            let new_lower_bound = apply_bound(lower_bound, category, value + 1);
            (new_lower_bound, upper_bound)
        }
        Rule::LessThan(category, value, _) => {
            let new_upper_bound = apply_bound(upper_bound, category, value - 1);
            (lower_bound, new_upper_bound)
        }
        Rule::Otherwise(_) => (lower_bound, upper_bound),
    }
}

fn rule_invert(rule: &Rule, lower_bound: Rating, upper_bound: Rating) -> (Rating, Rating) {
    match rule {
        Rule::LessThan(category, value, _) => {
            let new_lower_bound = apply_bound(lower_bound, category, *value);
            (new_lower_bound, upper_bound)
        }
        Rule::GreaterThan(category, value, _) => {
            let new_upper_bound = apply_bound(upper_bound, category, *value);
            (lower_bound, new_upper_bound)
        }
        Rule::Otherwise(_) => (lower_bound, upper_bound),
    }
}

fn rule_process(
    rule: &Rule,
    workflow_map: &HashMap<&str, Vec<Rule>>,
    lower_bound: Rating,
    upper_bound: Rating,
) -> u64 {
    let (lower_bound, upper_bound) = rule_apply(&rule, lower_bound, upper_bound);
    let action = match rule {
        Rule::GreaterThan(_, _, action) => action,
        Rule::LessThan(_, _, action) => action,
        Rule::Otherwise(action) => action,
    };
    match action {
        Action::Accept => combinations(lower_bound, upper_bound),
        Action::Reject => 0,
        Action::Goto(label) => workflow_run(label, workflow_map, lower_bound, upper_bound),
    }
}

fn workflow_run(
    label: &str,
    workflow_map: &HashMap<&str, Vec<Rule>>,
    lower_bound: Rating,
    upper_bound: Rating,
) -> u64 {
    let rules = workflow_map.get(label).unwrap();
    let mut total = 0;
    let mut lower_bound = lower_bound.clone();
    let mut upper_bound = upper_bound.clone();
    for rule in rules {
        total += rule_process(&rule, workflow_map, lower_bound, upper_bound);
        (lower_bound, upper_bound) = rule_invert(&rule, lower_bound, upper_bound);
    }
    total
}

fn combinations(lower_bound: Rating, upper_bound: Rating) -> u64 {
    let cool = (upper_bound.cool - lower_bound.cool) + 1;
    let musical = (upper_bound.musical - lower_bound.musical) + 1;
    let aerodynamic = (upper_bound.aerodynamic - lower_bound.aerodynamic) + 1;
    let shiny = (upper_bound.shiny - lower_bound.shiny) + 1;
    cool * musical * aerodynamic * shiny
}

fn part2(input: &str) -> u64 {
    let (workflow_map, _) = parse(input);
    let lower_bound = Rating {
        cool: 1,
        musical: 1,
        aerodynamic: 1,
        shiny: 1,
    };
    let upper_bound = Rating {
        cool: 4000,
        musical: 4000,
        aerodynamic: 4000,
        shiny: 4000,
    };
    workflow_run("in", &workflow_map, lower_bound, upper_bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        assert_eq!(
            parse_rule("a<2006:qkq"),
            Rule::LessThan(Category::Aerodynamic, 2006, Action::Goto("qkq"))
        );
        assert_eq!(
            parse_rule("m>2090:A"),
            Rule::GreaterThan(Category::Musical, 2090, Action::Accept)
        );
        assert_eq!(parse_rule("rfg"), Rule::Otherwise(Action::Goto("rfg")));
    }

    #[test]
    fn test_parse_rating() {
        assert_eq!(
            parse_rating("{x=787,m=2655,a=1222,s=2876}"),
            Rating {
                cool: 787,
                musical: 2655,
                aerodynamic: 1222,
                shiny: 2876
            }
        )
    }

    static SAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 19114)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 167409079868000)
    }
}
