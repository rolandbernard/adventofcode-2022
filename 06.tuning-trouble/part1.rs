use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let mut seen = [0; 26];
    for i in 0..3 {
        seen[input[i] as usize - 'a' as usize] += 1;
    }
    let mut result = None;
    for i in 3..input.len() {
        seen[input[i] as usize - 'a' as usize] += 1;
        if *seen.iter().max().unwrap() == 1 {
            result = Some(i);
            break;
        }
        seen[input[i - 3] as usize - 'a' as usize] -= 1;
    }
    println!("Result: {}", result.unwrap() + 1);
}
