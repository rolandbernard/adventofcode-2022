fn main() {
    let mut signal = 0;
    let mut x = 1;
    let mut cycle = 1;
    for line in std::io::stdin().lines() {
        if (cycle - 20) % 40 == 0 {
            signal += cycle * x;
        }
        let line = line.unwrap();
        let split = line.split(' ').collect::<Vec<_>>();
        match split[0] {
            "addx" => {
                if (cycle - 19) % 40 == 0 {
                    signal += (cycle + 1) * x;
                }
                cycle += 2;
                x += split[1].parse::<i64>().unwrap();
            }
            "noop" => {
                cycle += 1;
            }
            _ => panic!(),
        }
    }
    println!("Result: {signal}");
}
