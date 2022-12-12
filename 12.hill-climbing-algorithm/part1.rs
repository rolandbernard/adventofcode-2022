use std::{collections::BinaryHeap, cmp::Reverse};

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map: Vec<Vec<_>> = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, x)| {
                    (if x == 'S' {
                        start = (i, j);
                        'a'
                    } else if x == 'E' {
                        end = (i, j);
                        'z'
                    } else {
                        x
                    }) as u8
                })
                .collect()
        })
        .collect();
    let mut dist = vec![vec![usize::MAX; map[0].len()]; map.len()];
    dist[start.0][start.1] = 0;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));
    while let Some((Reverse(d), pos)) = queue.pop() {
        if pos == end {
            break;
        }
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if pos.0 as i64 + di >= 0
                && pos.0 as i64 + di < map.len() as i64
                && pos.1 as i64 + dj >= 0
                && pos.1 as i64 + dj < map[0].len() as i64
            {
                let nd = d + 1;
                let np = ((pos.0 as i64 + di) as usize, (pos.1 as i64 + dj) as usize);
                if map[pos.0][pos.1] + 1 >= map[np.0][np.1] && nd < dist[np.0][np.1] {
                    dist[np.0][np.1] = nd;
                    queue.push((Reverse(nd), np));
                }
            }
        }
    }
    println!("Result: {}", dist[end.0][end.1]);
}
