fn main() {
    let mut count = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let ranges: Vec<Vec<_>> = line
            .split(',')
            .map(|p| {
                p.split('-')
                    .map(str::parse::<i64>)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect();
        if ranges[0][0] <= ranges[1][0] && ranges[0][1] >= ranges[1][1]
            || ranges[0][0] >= ranges[1][0] && ranges[0][1] <= ranges[1][1]
        {
            count += 1;
        }
    }
    println!("Result: {}", count);
}
