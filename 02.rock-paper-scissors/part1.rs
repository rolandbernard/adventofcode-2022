
fn main() {
    let mut score = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let chars = line.chars().collect::<Vec<_>>();
        let elf = chars[0] as i64 - 'A' as i64;
        let you = chars[2] as i64 - 'X' as i64;
        score += you + 1;
        if elf == you {
            score += 3;
        } else if (elf + 1) % 3 == you {
            score += 6;
        }
    }
    println!("Result: {}", score);
}

