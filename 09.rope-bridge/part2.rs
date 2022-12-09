use std::collections::HashSet;

const KNOTS: usize = 10;

fn main() {
    let mut rope = [(0, 0); KNOTS];
    let mut visited = HashSet::new();
    visited.insert(rope[KNOTS - 1]);
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let split = line.split(' ').collect::<Vec<_>>();
        let dist = split[1].parse::<i64>().unwrap();
        match split[0] {
            "U" => {
                rope[0].1 += dist;
            }
            "D" => {
                rope[0].1 -= dist;
            }
            "R" => {
                rope[0].0 += dist;
            }
            "L" => {
                rope[0].0 -= dist;
            }
            _ => panic!(),
        }
        let mut change = true;
        while change {
            change = false;
            for i in 1..KNOTS {
                if i64::max(
                    (rope[i - 1].0 - rope[i].0).abs(),
                    (rope[i - 1].1 - rope[i].1).abs(),
                ) > 1
                {
                    rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                    change = true;
                    if i == KNOTS - 1 {
                        visited.insert(rope[i]);
                    }
                }
            }
        }
    }
    println!("Result: {}", visited.len());
}
