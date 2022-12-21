use std::collections::HashMap;

#[derive(Clone)]
enum MonkeyOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone)]
enum MonkeyInstr {
    Num(f64),
    Expr(MonkeyOp, String, String),
}

fn eval_monkey(name: &str, monkey: &HashMap<String, MonkeyInstr>, humn: f64) -> f64 {
    if name == "humn" {
        humn
    } else {
        match &monkey[name] {
            MonkeyInstr::Num(i) => *i,
            MonkeyInstr::Expr(op, l, r) => {
                let left = eval_monkey(l, monkey, humn);
                let right = eval_monkey(r, monkey, humn);
                match op {
                    MonkeyOp::Add => left + right,
                    MonkeyOp::Sub => left - right,
                    MonkeyOp::Mul => left * right,
                    MonkeyOp::Div => left / right,
                }
            }
        }
    }
}

fn contains_humn(name: &str, monkey: &HashMap<String, MonkeyInstr>) -> bool {
    if name == "humn" {
        true
    } else {
        match &monkey[name] {
            MonkeyInstr::Num(_) => false,
            MonkeyInstr::Expr(_, l, r) => contains_humn(l, monkey) || contains_humn(r, monkey),
        }
    }
}

fn make_equal(name: &str, monkey: &HashMap<String, MonkeyInstr>, value: f64) -> f64 {
    if name == "humn" {
        value
    } else {
        match &monkey[name] {
            MonkeyInstr::Num(_) => panic!(),
            MonkeyInstr::Expr(op, l, r) => match op {
                MonkeyOp::Add => {
                    if contains_humn(l, monkey) {
                        make_equal(l, monkey, value - eval_monkey(r, monkey, f64::NAN))
                    } else {
                        make_equal(r, monkey, value - eval_monkey(l, monkey, f64::NAN))
                    }
                }
                MonkeyOp::Sub => {
                    if contains_humn(l, monkey) {
                        make_equal(l, monkey, value + eval_monkey(r, monkey, f64::NAN))
                    } else {
                        make_equal(r, monkey, eval_monkey(l, monkey, f64::NAN) - value)
                    }
                }
                MonkeyOp::Mul => {
                    if contains_humn(l, monkey) {
                        make_equal(l, monkey, value / eval_monkey(r, monkey, f64::NAN))
                    } else {
                        make_equal(r, monkey, value / eval_monkey(l, monkey, f64::NAN))
                    }
                }
                MonkeyOp::Div => {
                    if contains_humn(l, monkey) {
                        make_equal(l, monkey, value * eval_monkey(r, monkey, f64::NAN))
                    } else {
                        make_equal(r, monkey, eval_monkey(l, monkey, f64::NAN) / value)
                    }
                }
            },
        }
    }
}

fn main() {
    let mut monkey = HashMap::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let split = line.split(": ").collect::<Vec<_>>();
        let equ = split[1].split(" ").collect::<Vec<_>>();
        monkey.insert(
            split[0].to_owned(),
            if equ.len() == 1 {
                MonkeyInstr::Num(equ[0].parse().unwrap())
            } else {
                MonkeyInstr::Expr(
                    match equ[1] {
                        "+" => MonkeyOp::Add,
                        "-" => MonkeyOp::Sub,
                        "*" => MonkeyOp::Mul,
                        "/" => MonkeyOp::Div,
                        _ => panic!(),
                    },
                    equ[0].to_owned(),
                    equ[2].to_owned(),
                )
            },
        );
    }
    if let MonkeyInstr::Expr(_, left, right) = monkey["root"].clone() {
        monkey.insert(
            "root".to_owned(),
            MonkeyInstr::Expr(MonkeyOp::Sub, left, right),
        );
    } else {
        panic!();
    }
    println!(
        "Result: {:?}",
        make_equal("root", &monkey, 0.0).round() as i64
    );
}
