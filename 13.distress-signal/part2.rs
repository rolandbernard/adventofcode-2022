use std::{collections::VecDeque, io::Read};

#[derive(PartialEq, Debug, Eq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.partial_cmp(b),
            (Packet::Int(i), Packet::List(_)) => {
                Packet::List(vec![Packet::Int(*i)]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Int(i)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Int(*i)]))
            }
            (Packet::List(a), Packet::List(b)) => {
                for (ap, bp) in a.iter().zip(b.iter()) {
                    if let Some(ord) = ap.partial_cmp(bp) {
                        if !ord.is_eq() {
                            return Some(ord);
                        }
                    }
                }
                return a.len().partial_cmp(&b.len());
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_packet(input: &mut VecDeque<char>) -> Packet {
    if *input.front().unwrap() == '[' {
        let mut values = Vec::new();
        input.pop_front();
        while *input.front().unwrap() != ']' {
            if *input.front().unwrap() == ',' {
                input.pop_front();
            }
            values.push(parse_packet(input));
        }
        input.pop_front();
        return Packet::List(values);
    } else {
        let mut value = 0;
        while input.front().unwrap().is_ascii_digit() {
            value *= 10;
            value += input.pop_front().unwrap().to_digit(10).unwrap();
        }
        return Packet::Int(value);
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut packets: Vec<_> = input
        .split("\n")
        .filter(|p| !p.is_empty())
        .map(|p| parse_packet(&mut p.chars().collect()))
        .collect();
    packets.sort();
    let mut prod = 1;
    let mut divs = VecDeque::from([
        parse_packet(&mut "[[2]]".chars().collect()),
        parse_packet(&mut "[[6]]".chars().collect()),
    ]);
    let mut i = 1;
    for p in packets {
        if divs.front().map_or(false, |d| d < &p) {
            divs.pop_front();
            prod *= i;
            i += 1;
        }
        i += 1;
    }
    for _ in divs {
        prod *= i;
        i += 1;
    }
    println!("Result: {}", prod);
}
