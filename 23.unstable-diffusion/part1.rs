use std::collections::{HashMap, HashSet};

const DIRS: [[(i64, i64); 3]; 4] = [
    [(-1, 0), (-1, 1), (-1, -1)],
    [(1, 0), (1, 1), (1, -1)],
    [(0, -1), (1, -1), (-1, -1)],
    [(0, 1), (1, 1), (-1, 1)],
];

fn round(elves: &mut HashSet<(i64, i64)>, iter: usize) {
    let mut moves = HashMap::new();
    for &(i, j) in elves.iter() {
        let close = DIRS
            .iter()
            .flatten()
            .map(|&(di, dj)| (i + di, j + dj))
            .any(|x| elves.contains(&x));
        if close {
            for d in 0..4 {
                let d = (d + iter) % 4;
                let free = DIRS[d]
                    .iter()
                    .map(|&(di, dj)| (i + di, j + dj))
                    .all(|x| !elves.contains(&x));
                if free {
                    let next = (i + DIRS[d][0].0, j + DIRS[d][0].1);
                    if moves.contains_key(&next) {
                        moves.insert(next, None);
                    } else {
                        moves.insert(next, Some((i, j)));
                    }
                    break;
                }
            }
        }
    }
    for (to, from) in moves {
        if let Some(from) = from {
            elves.remove(&from);
            elves.insert(to);
        }
    }
}

fn main() {
    let mut elves = HashSet::new();
    for (i, line) in std::io::stdin().lines().enumerate() {
        for (j, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                elves.insert((i as i64, j as i64));
            }
        }
    }
    for i in 0..10 {
        round(&mut elves, i);
    }
    let min = elves
        .iter()
        .fold((i64::MAX, i64::MAX), |m, e| (m.0.min(e.0), m.1.min(e.1)));
    let max = elves
        .iter()
        .fold((i64::MIN, i64::MIN), |m, e| (m.0.max(e.0), m.1.max(e.1)));
    let empty = (max.0 - min.0 + 1) * (max.1 - min.1 + 1) - elves.len() as i64;
    println!("Result: {:?}", empty);
}
