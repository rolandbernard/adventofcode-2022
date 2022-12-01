
fn main() {
    let mut cur = 0;
    let mut max = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            max = max.max(cur);
            cur = 0;
        } else {
            cur += line.parse::<i64>().unwrap();
        }
    }
    println!("Result: {}", max.max(cur));
}

