use std::fs;
use std::io;
use z3::ast::Ast;

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)?;
    let input = file_contents.as_str();

    println!(
        "Part 1: {}",
        part1(input, 200_000_000_000_000, 400_000_000_000_000)
    );
    println!("Part 2: {}", part2(input));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hailstone {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

fn parse_line(line: &str) -> Hailstone {
    let numbers: Vec<i64> = line
        .split(|c: char| !c.is_digit(10) && c != '-')
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    Hailstone {
        position: (numbers[0], numbers[1], numbers[2]),
        velocity: (numbers[3], numbers[4], numbers[5]),
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn part1(input: &str, min: i64, max: i64) -> usize {
    // https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
    fn cross(v: (i64, i64), w: (i64, i64)) -> i64 {
        v.0 * w.1 - v.1 * w.0
    }
    fn intersection(a: Hailstone, b: Hailstone) -> Option<(i64, i64)> {
        let p = (a.position.0, a.position.1);
        let r = (a.velocity.0, a.velocity.1);
        let q = (b.position.0, b.position.1);
        let s = (b.velocity.0, b.velocity.1);
        let rxs = cross(r, s);
        if rxs == 0 {
            return None;
        }
        let q_minus_p = (q.0 - p.0, q.1 - p.1);
        let t = cross(q_minus_p, s) as f64 / rxs as f64;
        let u = cross(q_minus_p, r) as f64 / rxs as f64;
        if t < 0_f64 || u < 0_f64 {
            return None;
        }
        Some((
            (p.0 as f64 + (t * r.0 as f64)) as i64,
            (p.1 as f64 + (t * r.1 as f64)) as i64,
        ))
    }
    let hailstones = parse(input);
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let a = hailstones[i];
            let b = hailstones[j];
            if let Some((x, y)) = intersection(a, b) {
                if min <= x && x <= max && min <= y && y <= max {
                    count += 1
                }
            }
        }
    }
    count
}

fn part2(input: &str) -> i64 {
    let hailstones = parse(input);
    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);

    let x = z3::ast::Int::new_const(&context, "x");
    let y = z3::ast::Int::new_const(&context, "y");
    let z = z3::ast::Int::new_const(&context, "z");
    let vx = z3::ast::Int::new_const(&context, "vx");
    let vy = z3::ast::Int::new_const(&context, "vy");
    let vz = z3::ast::Int::new_const(&context, "vz");

    for (i, hs) in hailstones.iter().take(3).enumerate() {
        let a = z3::ast::Int::from_i64(&context, hs.position.0);
        let va = z3::ast::Int::from_i64(&context, hs.velocity.0);
        let b = z3::ast::Int::from_i64(&context, hs.position.1);
        let vb = z3::ast::Int::from_i64(&context, hs.velocity.1);
        let c = z3::ast::Int::from_i64(&context, hs.position.2);
        let vc = z3::ast::Int::from_i64(&context, hs.velocity.2);

        let t = z3::ast::Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }
    if solver.check() == z3::SatResult::Sat {
        let m = solver.get_model().unwrap();
        return m.eval(&(x + y + z), true).unwrap().as_i64().unwrap();
    }
    panic!("Failed to solve!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("19, 13, 30 @ -2,  1, -2"),
            Hailstone {
                position: (19, 13, 30),
                velocity: (-2, 1, -2)
            }
        )
    }

    static SAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE, 7, 27), 2)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 47)
    }
}
