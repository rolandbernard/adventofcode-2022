use std::collections::HashSet;

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
    let mut shapes = shapes.iter().cycle();
    let sequence = std::io::stdin()
        .lines()
        .fold(String::new(), |s, e| s + e.unwrap().as_str());
    let mut sequence = sequence.chars().cycle();
    let mut fallen = HashSet::new();
    let mut max = 0;
    for _ in 0..2022 {
        let shape = shapes.next().unwrap();
        let mut side = true;
        let mut pos = (0, max + 3);
        loop {
            let next;
            if side {
                if sequence.next().unwrap() == '<' {
                    next = (pos.0 - 1, pos.1);
                } else {
                    next = (pos.0 + 1, pos.1);
                }
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
    }
    println!("Result: {}", max);
}
