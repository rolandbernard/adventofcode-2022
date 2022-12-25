fn read_snafu(str: &str) -> i64 {
    let mut num = 0;
    for c in str.chars() {
        num *= 5;
        num += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        }
    }
    return num;
}

fn write_snafu(num: i64) -> String {
    if num == 0 {
        return String::new();
    } else {
        let rest = num % 5;
        let prefix;
        if rest <= 2 {
            prefix = write_snafu(num / 5);
        } else {
            prefix = write_snafu(num / 5 + 1);
        }
        return prefix
            + match rest {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "=",
                4 => "-",
                _ => panic!(),
            };
    }
}

fn main() {
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        sum += read_snafu(&line);
    }
    println!("Result: {}", write_snafu(sum));
}
