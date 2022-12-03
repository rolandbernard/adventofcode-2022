use std::collections::HashSet;

fn to_priority(c: char) -> i64 {
    match c {
        'a'..='z' => c as i64 - 'a' as i64 + 1,
        'A'..='Z' => c as i64 - 'A' as i64 + 27,
        _ => unreachable!(),
    }
}

fn main() {
    let mut score = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let first = line.chars().take(line.len() / 2).collect::<HashSet<_>>();
        let second = line.chars().skip(line.len() / 2).collect::<HashSet<_>>();
        let common = *first.intersection(&second).next().unwrap();
        score += to_priority(common);
    }
    println!("Result: {}", score);
}
