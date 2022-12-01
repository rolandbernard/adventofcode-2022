
fn main() {
    let mut all = Vec::new();
    let mut cur = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            all.push(cur);
            cur = 0;
        } else {
            cur += line.parse::<i64>().unwrap();
        }
    }
    all.push(cur);
    all.sort_by_key(|x| -x);
    println!("Result: {}", all[..3].iter().sum::<i64>());
}

