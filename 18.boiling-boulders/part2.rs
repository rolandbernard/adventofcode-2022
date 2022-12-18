use std::collections::{HashSet, VecDeque};

const DIRS: [(i64, i64, i64); 6] = [
    (0, 0, 1),
    (0, 0, -1),
    (0, 1, 0),
    (0, -1, 0),
    (1, 0, 0),
    (-1, 0, 0),
];

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
    let min = cubes.iter().fold((i64::MAX, i64::MAX, i64::MAX), |s, e| {
        (s.0.min(e.0 - 1), s.1.min(e.1 - 1), s.2.min(e.2 - 1))
    });
    let max = cubes.iter().fold((i64::MIN, i64::MIN, i64::MIN), |s, e| {
        (s.0.max(e.0 + 1), s.1.max(e.1 + 1), s.2.max(e.2 + 1))
    });
    let mut outside = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(min);
    outside.insert(min);
    while let Some((x, y, z)) = queue.pop_front() {
        for (dx, dy, dz) in DIRS {
            let n = (x + dx, y + dy, z + dz);
            if min.0 <= n.0
                && min.1 <= n.1
                && min.2 <= n.2
                && n.0 <= max.0
                && n.1 <= max.1
                && n.2 <= max.2
                && !cubes.contains(&n)
                && !outside.contains(&n)
            {
                outside.insert(n);
                queue.push_back(n);
            }
        }
    }
    let mut sides = 0;
    for (x, y, z) in cubes.clone() {
        for (dx, dy, dz) in DIRS {
            if outside.contains(&(x + dx, y + dy, z + dz)) {
                sides += 1;
            }
        }
    }
    println!("Result: {}", sides);
}
