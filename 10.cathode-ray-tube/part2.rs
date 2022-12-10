fn main() {
    let mut pixels = Vec::new();
    let mut x = 1;
    let mut draw_pixel = |x: i64| {
        let diff = x - (pixels.len() as i64 % 40);
        pixels.push(diff.abs() <= 1);
    };
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let split = line.split(' ').collect::<Vec<_>>();
        draw_pixel(x);
        match split[0] {
            "addx" => {
                draw_pixel(x);
                x += split[1].parse::<i64>().unwrap();
            }
            "noop" => {
                // Do nothing.
            }
            _ => panic!(),
        }
    }
    println!("Result:");
    for j in 0..pixels.len() / 40 {
        for i in 0..40 {
            if pixels[40 * j + i] {
                print!("#");
            } else {
                print!("\x1b[90m.\x1b[m");
            }
        }
        println!();
    }
}
