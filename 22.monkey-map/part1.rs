use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut map = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = map.iter().map(Vec::len).max().unwrap();
    for row in &mut map {
        row.resize(width, ' ');
    }
    let instr = input
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .split_inclusive(|c| c == 'R' || c == 'L')
        .collect::<Vec<_>>();
    let mut pos = 'pos: loop {
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == '.' {
                    break 'pos (i as i64, j as i64);
                }
            }
        }
        panic!();
    };
    let mut dir = (0, 1);
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
            let mut ny = (pos.0 + dir.0).rem_euclid(map.len() as i64);
            let mut nx = (pos.1 + dir.1).rem_euclid(map[ny as usize].len() as i64);
            while map[ny as usize][nx as usize].is_whitespace() {
                ny = (ny + dir.0).rem_euclid(map.len() as i64);
                nx = (nx + dir.1).rem_euclid(map[ny as usize].len() as i64);
            }
            if map[ny as usize][nx as usize] == '.' {
                pos = (ny, nx);
            }
        }
        match rot {
            Some('L') => dir = (-dir.1, dir.0),
            Some('R') => dir = (dir.1, -dir.0),
            _ => { /* ignore */ }
        }
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
