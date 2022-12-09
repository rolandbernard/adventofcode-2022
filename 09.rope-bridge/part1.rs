use std::collections::HashSet;

fn main() {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(tail);
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let split = line.split(' ').collect::<Vec<_>>();
        let dist = split[1].parse::<i64>().unwrap();
        match split[0] {
            "U" => {
                head.1 += dist;
            }
            "D" => {
                head.1 -= dist;
            }
            "R" => {
                head.0 += dist;
            }
            "L" => {
                head.0 -= dist;
            }
            _ => panic!(),
        }
        while i64::max((head.0 - tail.0).abs(), (head.1 - tail.1).abs()) > 1 {
            tail.0 += (head.0 - tail.0).signum();
            tail.1 += (head.1 - tail.1).signum();
            visited.insert(tail);
        }
    }
    println!("Result: {}", visited.len());
}
