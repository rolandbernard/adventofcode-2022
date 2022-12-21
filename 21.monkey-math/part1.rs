use std::collections::HashMap;

enum MonkeyOp {
    Add,
    Sub,
    Mul,
    Div,
}

enum MonkeyInstr {
    Num(i64),
    Expr(MonkeyOp, String, String),
}

fn eval_monkey(name: &str, monkey: &HashMap<String, MonkeyInstr>) -> i64 {
    match &monkey[name] {
        MonkeyInstr::Num(i) => *i,
        MonkeyInstr::Expr(op, l, r) => {
            let left = eval_monkey(l, monkey);
            let right = eval_monkey(r, monkey);
            match op {
                MonkeyOp::Add => left + right,
                MonkeyOp::Sub => left - right,
                MonkeyOp::Mul => left * right,
                MonkeyOp::Div => left / right,
            }
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
    println!("Result: {:?}", eval_monkey("root", &monkey));
}
