use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut blizzards = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut start = 0;
    let mut end = 0;
    for (i, line) in std::io::stdin().lines().enumerate() {
        width = 0;
        for (j, c) in line.unwrap().chars().enumerate() {
            if i == 0 && c == '.' {
                start = j as i64 - 1;
            } else if c == '.' {
                end = j as i64 - 1;
            } else if c == '>' {
                blizzards.push(((i as i64 - 1, j as i64 - 1), (0, 1)));
            } else if c == 'v' {
                blizzards.push(((i as i64 - 1, j as i64 - 1), (1, 0)));
            } else if c == '<' {
                blizzards.push(((i as i64 - 1, j as i64 - 1), (0, -1)));
            } else if c == '^' {
                blizzards.push(((i as i64 - 1, j as i64 - 1), (-1, 0)));
            }
            width += 1;
        }
        height += 1;
    }
    height -= 2;
    width -= 2;
    let period = (height * width / gcd(height, width)) as usize;
    let mut history = Vec::new();
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();
    dist.insert((0, (-1, start, 0)), 0);
    queue.push(Reverse((0, 0, (-1, start, 0))));
    while let Some(Reverse((_, t, (y, x, z)))) = queue.pop() {
        if y == height && z == 2 {
            println!("Result: {:?}", t);
            break;
        } else {
            while history.len() <= t + 1 {
                let time = history.len() as i64;
                history.push(
                    blizzards
                        .iter()
                        .map(move |&((y, x), (dy, dx))| {
                            (
                                (y + dy * time).rem_euclid(height),
                                (x + dx * time).rem_euclid(width),
                            )
                        })
                        .collect::<HashSet<_>>(),
                )
            }
            let bliz = &history[t + 1];
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0
                    && (ny >= 0 || (nx == start && ny == -1))
                    && nx < width
                    && (ny < height || (nx == end && ny == height))
                    && !bliz.contains(&(ny, nx))
                {
                    let mut nz = z;
                    if z == 0 && ny == height {
                        nz = 1;
                    } else if z == 1 && ny == -1 {
                        nz = 2;
                    }
                    let key = ((t + 1) % period, (ny, nx, nz));
                    let old_dist = dist.get(&key).cloned().unwrap_or(usize::MAX);
                    if t + 1 < old_dist {
                        dist.insert(key, t + 1);
                        queue.push(Reverse((
                            t as i64
                                + 1
                                + (height - ny).abs()
                                + (width - nx).abs()
                                + (2 - nz) * (width + height + 2),
                            t + 1,
                            (ny, nx, nz),
                        )));
                    }
                }
            }
        }
    }
}
