use std::collections::{HashMap, HashSet};

const DIRS: [[(i64, i64); 3]; 4] = [
    [(-1, 0), (-1, 1), (-1, -1)],
    [(1, 0), (1, 1), (1, -1)],
    [(0, -1), (1, -1), (-1, -1)],
    [(0, 1), (1, 1), (-1, 1)],
];

fn round(elves: &mut HashSet<(i64, i64)>, iter: usize) -> bool {
    let mut moves = HashMap::with_capacity(elves.len());
    for &(i, j) in elves.iter() {
        let close = (-1..=1)
            .flat_map(|di| (-1..=1).map(move |dj| (di, dj)))
            .filter(|&(di, dj)| di != 0 || dj != 0)
            .map(|(di, dj)| (i + di, j + dj))
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
    let mut moved = false;
    for (&to, from) in &moves {
        if let Some(from) = from {
            elves.remove(&from);
            elves.insert(to);
            moved = true;
        }
    }
    return moved;
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
    let mut iter = 0;
    while round(&mut elves, iter) {
        iter += 1;
    }
    println!("Result: {:?}", iter + 1);
}
