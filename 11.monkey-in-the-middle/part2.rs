use std::{cmp::Reverse, collections::VecDeque, io::Read};

enum MonkeyOp {
    Add(i64),
    Mul(i64),
    Sqr,
}

struct Monkey {
    items: VecDeque<i64>,
    op: MonkeyOp,
    cond: i64,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

fn main() {
    let mut monkeys = Vec::new();
    let mut input = String::new();
    let mut modulo = 1;
    std::io::stdin().read_to_string(&mut input).unwrap();
    for desc in input.trim().split("\n\n") {
        let lines = desc
            .trim()
            .lines()
            .map(|l| l.split(":").skip(1).next().unwrap().trim())
            .collect::<Vec<_>>();
        let items = lines[1].split(", ").map(|x| x.parse().unwrap()).collect();
        let expr = lines[2].split(" = ").skip(1).next().unwrap();
        let op;
        if expr.contains('+') {
            let operands = expr.split(" + ").collect::<Vec<_>>();
            op = MonkeyOp::Add(operands[1].parse().unwrap());
        } else {
            let operands = expr.split(" * ").collect::<Vec<_>>();
            if operands[1] == "old" {
                op = MonkeyOp::Sqr;
            } else {
                op = MonkeyOp::Mul(operands[1].parse().unwrap());
            }
        }
        let cond = lines[3].split(' ').skip(2).next().unwrap().parse().unwrap();
        modulo *= cond;
        let if_true = lines[4].split(' ').skip(3).next().unwrap().parse().unwrap();
        let if_false = lines[5].split(' ').skip(3).next().unwrap().parse().unwrap();
        monkeys.push(Monkey {
            items,
            op,
            cond,
            if_true,
            if_false,
            inspections: 0,
        });
    }
    for _ in 0..10_000 {
        for monkey in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey].items.pop_front() {
                monkeys[monkey].inspections += 1;
                let new = match monkeys[monkey].op {
                    MonkeyOp::Add(n) => item + n,
                    MonkeyOp::Mul(n) => item * n,
                    MonkeyOp::Sqr => item * item,
                };
                let new = new % modulo;
                let target;
                if new % monkeys[monkey].cond == 0 {
                    target = monkeys[monkey].if_true;
                } else {
                    target = monkeys[monkey].if_false;
                }
                monkeys[target].items.push_back(new);
            }
        }
    }
    monkeys.sort_by_key(|m| Reverse(m.inspections));
    let business = monkeys
        .iter()
        .map(|m| m.inspections)
        .take(2)
        .product::<usize>();
    println!("Result: {}", business);
}
