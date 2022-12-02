
fn main() {
    let mut score = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let chars = line.chars().collect::<Vec<_>>();
        let elf = chars[0] as i64 - 'A' as i64;
        match chars[2] {
            'X' => score += (elf + 2) % 3 + 1,
            'Y' => score += elf + 4,
            'Z' => score += (elf + 1) % 3 + 7,
            _ => unreachable!(),
        }
    }
    println!("Result: {}", score);
}

