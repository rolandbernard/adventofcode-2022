use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let mut seen = [0; 26];
    for i in 0..13 {
        seen[input[i] as usize - 'a' as usize] += 1;
    }
    let mut result = None;
    for i in 13..input.len() {
        seen[input[i] as usize - 'a' as usize] += 1;
        if *seen.iter().max().unwrap() == 1 {
            result = Some(i);
            break;
        }
        seen[input[i - 13] as usize - 'a' as usize] -= 1;
    }
    println!("Result: {}", result.unwrap() + 1);
}
