const ROW: i64 = 2_000_000;

fn cells(ranges: &[(i64, i64)]) -> i64 {
    if ranges.is_empty() {
        return 0;
    } else {
        let head = ranges[0];
        let mut inter = Vec::new();
        for range in &ranges[1..] {
            let start = i64::max(head.0, range.0);
            let end = i64::min(head.1, range.1);
            if start <= end {
                inter.push((start, end));
            }
        }
        return head.1 - head.0 + cells(&ranges[1..]) - cells(&inter);
    }
}

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
    let ranges = sensors
        .iter()
        .map(|(x, y, r)| (x, r - (y - ROW).abs()))
        .filter(|(_, r)| *r >= 0)
        .map(|(x, r)| (x - r, x + r))
        .collect::<Vec<_>>();
    println!("Result: {}", cells(&ranges));
}
