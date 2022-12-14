use std::collections::HashSet;

fn main() {
    let mut blocked = HashSet::new();
    for l in std::io::stdin().lines() {
        let line = l.unwrap();
        let points: Vec<Vec<i64>> = line
            .split(" -> ")
            .map(|p| p.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();
        for (from, to) in points.iter().zip(points.iter().skip(1)) {
            let dx = (to[0] - from[0]).signum();
            let dy = (to[1] - from[1]).signum();
            let mut cur = (from[0], from[1]);
            blocked.insert(cur);
            while cur.0 != to[0] || cur.1 != to[1] {
                cur.0 += dx;
                cur.1 += dy;
                blocked.insert(cur);
            }
        }
    }
    let max_y = blocked.iter().map(|p| p.1).max().unwrap();
    let mut dropped = 0;
    'outer: loop {
        let mut pos = (500, 0);
        'inner: loop {
            if pos.1 >= max_y + 1 {
                break 'inner;
            }
            for next in [
                (pos.0, pos.1 + 1),
                (pos.0 - 1, pos.1 + 1),
                (pos.0 + 1, pos.1 + 1),
            ] {
                if !blocked.contains(&next) {
                    pos = next;
                    continue 'inner;
                }
            }
            break;
        }
        blocked.insert(pos);
        dropped += 1;
        if pos == (500, 0) {
            break 'outer;
        }
    }
    println!("Result: {}", dropped);
}
