fn shift(prev: &mut [usize], next: &mut [usize], i: usize, shift: i64) {
    for _ in 0..shift {
        let p = prev[i];
        let n = next[i];
        next[p] = n;
        prev[n] = p;
        next[i] = next[n];
        prev[i] = n;
        prev[next[i]] = i;
        next[n] = i;
    }
}

fn decrypt(file: &[i64]) -> Vec<i64> {
    let cycle = file.len() as i64 - 1;
    let mut prev = Vec::with_capacity(file.len());
    let mut next = Vec::with_capacity(file.len());
    for i in 0..file.len() {
        prev.push((file.len() + i - 1) % file.len());
        next.push((i + 1) % file.len());
    }
    for i in 0..file.len() {
        let right_shift = (file[i] % cycle + cycle) % cycle;
        if cycle - right_shift < right_shift {
            shift(&mut next, &mut prev, i, cycle - right_shift);
        } else {
            shift(&mut prev, &mut next, i, right_shift);
        }
    }
    let mut decrypted = Vec::with_capacity(file.len());
    let mut pos = file.iter().position(|&x| x == 0).unwrap();
    for _ in 0..file.len() {
        decrypted.push(file[pos]);
        pos = next[pos];
    }
    return decrypted;
}

fn main() {
    let file = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect::<Vec<i64>>();
    let plain = decrypt(&file);
    let coordinate =
        plain[1000 % plain.len()] + plain[2000 % plain.len()] + plain[3000 % plain.len()];
    println!("Result: {coordinate:?}");
}
