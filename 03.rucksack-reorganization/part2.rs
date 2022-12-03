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
    let mut last = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap().chars().collect::<HashSet<_>>();
        last.push(line);
        if last.len() == 3 {
            let common = *last[0]
                .intersection(&last[1])
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&last[2])
                .next()
                .unwrap();
            score += to_priority(common);
            last.clear();
        }
    }
    println!("Result: {}", score);
}
