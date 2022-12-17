use std::collections::{HashMap, HashSet};

const N: usize = 1_000_000_000_000;

fn collide(fixed: &HashSet<(i64, i64)>, tile: &HashSet<(i64, i64)>, dx: i64, dy: i64) -> bool {
    tile.iter()
        .any(|&(x, y)| y + dy < 0 || x + dx < 0 || x + dx >= 7 || fixed.contains(&(x + dx, y + dy)))
}

fn main() {
    let shapes = vec![
        HashSet::from([(2, 0), (3, 0), (4, 0), (5, 0)]),
        HashSet::from([(2, 1), (3, 0), (4, 1), (3, 1), (3, 2)]),
        HashSet::from([(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)]),
        HashSet::from([(2, 0), (2, 1), (2, 2), (2, 3)]),
        HashSet::from([(2, 0), (3, 0), (2, 1), (3, 1)]),
    ];
    let mut shapes_iter = shapes.iter().cycle();
    let sequence = std::io::stdin()
        .lines()
        .fold(String::new(), |s, e| s + e.unwrap().as_str());
    let mut sequence_iter = sequence.chars().cycle();
    let mut fallen = HashSet::new();
    let mut max = 0;
    let mut max_col = [0; 7];
    let mut iter = 0;
    let mut step = 0;
    let mut seen = HashMap::new();
    let mut skipped = 0;
    while iter < N {
        if skipped == 0 {
            let mut offsets = [0; 7];
            for i in 0..7 {
                offsets[i] = max - max_col[i];
            }
            let mut valid = true;
            for i in 0..7 {
                let mut most = offsets[i];
                if i > 0 && offsets[i - 1] > most {
                    most = offsets[i - 1];
                }
                if i < 6 && offsets[i + 1] > most {
                    most = offsets[i + 1];
                }
                for j in offsets[i] + 1..=most {
                    if !fallen.contains(&(i as i64, max - j)) {
                        valid = false;
                    }
                }
            }
            if valid {
                let state = (offsets, step % sequence.len(), iter % shapes.len());
                if seen.contains_key(&state) {
                    let last: (usize, i64) = seen[&state];
                    let max_change = max - last.1;
                    let cycle = iter - last.0;
                    let fit = (N - iter) / cycle;
                    iter += cycle * fit;
                    skipped = max_change * fit as i64;
                } else {
                    seen.insert(state, (iter, max));
                }
            }
        }
        let shape = shapes_iter.next().unwrap();
        let mut side = true;
        let mut pos = (0, max + 3);
        loop {
            let next;
            if side {
                if sequence_iter.next().unwrap() == '<' {
                    next = (pos.0 - 1, pos.1);
                } else {
                    next = (pos.0 + 1, pos.1);
                }
                step += 1;
            } else {
                next = (pos.0, pos.1 - 1);
            }
            if !collide(&fallen, shape, next.0, next.1) {
                pos = next;
            } else if !side {
                break;
            }
            side = !side;
        }
        fallen.extend(shape.iter().map(|&(x, y)| (x + pos.0, y + pos.1)));
        max = i64::max(
            max,
            shape.iter().map(|&(_, y)| y + pos.1 + 1).max().unwrap(),
        );
        for &(x, y) in shape {
            let x = x + pos.0;
            let y = y + pos.1;
            if y + 1 > max {
                max = y + 1;
            }
            if y + 1 > max_col[x as usize] {
                max_col[x as usize] = y + 1;
            }
        }
        iter += 1;
    }
    println!("Result: {}", max + skipped);
}
