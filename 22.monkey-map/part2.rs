use std::{collections::HashMap, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let map = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let instr = input
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .split_inclusive(|c| c == 'R' || c == 'L')
        .collect::<Vec<_>>();
    let mut pos = (0, map[0].iter().position(|&x| x == '.').unwrap() as i64);
    let mut dir = (0, 1);
    let mut path = HashMap::new();
    for inst in instr {
        let mut mov = 0;
        let mut rot = None;
        for c in inst.chars() {
            if let Some(d) = c.to_digit(10) {
                mov *= 10;
                mov += d;
            } else {
                rot = Some(c);
            }
        }
        for _ in 0..mov {
            path.insert(pos, dir);
            let mut ny = pos.0 + dir.0;
            let mut nx = pos.1 + dir.1;
            let mut nd = dir;
            if nx < 0
                || ny < 0
                || ny >= map.len() as i64
                || nx >= map[ny as usize].len() as i64
                || map[ny as usize][nx as usize].is_whitespace()
            {
                ny = pos.0;
                nx = pos.1;
                (ny, nx, nd) = match (ny / 50, nx / 50, nd) {
                    (0, 1, (-1, 0)) => (100 + nx, 0, (0, 1)),
                    (0, 1, (0, -1)) => (149 - ny, 0, (0, 1)),
                    (0, 2, (-1, 0)) => (199, nx - 100, (-1, 0)),
                    (0, 2, (0, 1)) => (149 - ny, 99, (0, -1)),
                    (0, 2, (1, 0)) => (nx - 50, 99, (0, -1)),
                    (1, 1, (0, 1)) => (49, 50 + ny, (-1, 0)),
                    (1, 1, (0, -1)) => (100, ny - 50, (1, 0)),
                    (2, 0, (-1, 0)) => (50 + nx, 50, (0, 1)),
                    (2, 0, (0, -1)) => (149 - ny, 50, (0, 1)),
                    (2, 1, (0, 1)) => (149 - ny, 149, (0, -1)),
                    (2, 1, (1, 0)) => (100 + nx, 49, (0, -1)),
                    (3, 0, (0, 1)) => (149, ny - 100, (-1, 0)),
                    (3, 0, (1, 0)) => (0, 100 + nx, (1, 0)),
                    (3, 0, (0, -1)) => (0, ny - 100, (1, 0)),
                    _ => panic!(),
                };
            }
            if map[ny as usize][nx as usize] == '.' {
                pos = (ny, nx);
                dir = nd;
            } else {
                break;
            }
        }
        match rot {
            Some('L') => dir = (-dir.1, dir.0),
            Some('R') => dir = (dir.1, -dir.0),
            _ => { /* ignore */ }
        }
        path.insert(pos, dir);
    }
    let result = 1000 * (pos.0 + 1)
        + 4 * (pos.1 + 1)
        + match dir {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => panic!(),
        };
    println!("Result: {:?}", result);
}
