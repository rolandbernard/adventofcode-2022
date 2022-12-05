fn main() {
    let mut moves = false;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            for stack in &mut stacks {
                stack.reverse();
            }
            moves = true;
        } else if moves {
            let parts = line.split(' ').collect::<Vec<_>>();
            let num = parts[1].parse::<usize>().unwrap();
            let from = parts[3].parse::<usize>().unwrap() - 1;
            let to = parts[5].parse::<usize>().unwrap() - 1;
            let new_len = stacks[from].len() - num;
            let tmp = stacks[from][new_len..].to_vec();
            stacks[from].resize_with(new_len, || unreachable!());
            stacks[to].extend(tmp);
        } else if line.contains('[') {
            let num_stack = (line.len() + 1) / 4;
            if stacks.len() < num_stack {
                stacks.resize(num_stack, Vec::new());
            }
            let chars = line.chars().collect::<Vec<_>>();
            for i in 0..num_stack {
                if !chars[1 + 4 * i].is_whitespace() {
                    stacks[i].push(chars[1 + 4 * i]);
                }
            }
        }
    }
    println!(
        "Result: {}",
        stacks.iter().map(|e| e.last().unwrap()).collect::<String>()
    );
}
