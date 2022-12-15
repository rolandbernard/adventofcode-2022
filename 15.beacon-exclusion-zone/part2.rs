const MAX_CORD: i64 = 4_000_000;

fn main() {
    let mut sensors = Vec::new();
    for l in std::io::stdin().lines() {
        let line = l.unwrap();
        let parts = line
            .split(|c| c == ' ' || c == ':' || c == '=' || c == ',')
            .collect::<Vec<_>>();
        let sx = parts[3].parse::<i64>().unwrap();
        let sy = parts[6].parse::<i64>().unwrap();
        let bx = parts[13].parse::<i64>().unwrap();
        let by = parts[16].parse::<i64>().unwrap();
        sensors.push((sx, sy, (sx - bx).abs() + (sy - by).abs()));
    }
    sensors.sort();
    for ty in 0..=MAX_CORD {
        let mut tx = 0;
        for s in &sensors {
            let r = s.2 - (s.1 - ty).abs();
            let start = s.0 - r;
            let end = s.0 + r;
            if start <= tx && end >= tx {
                tx = end + 1;
                if tx > MAX_CORD {
                    break;
                }
            }
        }
        if tx <= MAX_CORD {
            println!("Result: {}", 4_000_000 * tx + ty);
            break;
        }
    }
}
