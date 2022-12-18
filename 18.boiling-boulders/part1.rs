use std::collections::HashSet;

fn main() {
    let mut cubes = HashSet::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let split = line
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<i64>>();
        cubes.insert((split[0], split[1], split[2]));
    }
    let mut sides = 0;
    for (x, y, z) in cubes.clone() {
        for (dx, dy, dz) in [
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ] {
            if !cubes.contains(&(x + dx, y + dy, z + dz)) {
                sides += 1;
            }
        }
    }
    println!("Result: {}", sides);
}
