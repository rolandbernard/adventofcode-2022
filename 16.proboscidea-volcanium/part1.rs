use std::{
    cmp::Reverse,
    collections::{HashMap, VecDeque},
};

fn find_max(
    flows: &[i64],
    dists: &[Vec<i64>],
    pos: usize,
    time: i64,
    open: u64,
    already: i64,
    flow: i64,
    bound: i64,
) -> i64 {
    if already + time * flow <= bound {
        return bound;
    } else if time == 0 || flow == 0 {
        return already;
    } else {
        let mut res = i64::max(bound, already);
        for n in 0..flows.len() {
            if (open >> n) & 1 == 0 && dists[pos][n] < time {
                let new_time = time - dists[pos][n] - 1;
                let this = find_max(
                    flows,
                    dists,
                    n,
                    new_time,
                    open | (1 << n),
                    already + new_time * flows[n],
                    flow - flows[n],
                    res,
                );
                if this > res {
                    res = this;
                }
            }
        }
        return res;
    }
}

fn dist_from_to(graph: &[Vec<usize>], start: usize, end: usize) -> i64 {
    let mut dist = vec![i64::MAX; graph.len()];
    let mut queue = VecDeque::new();
    dist[start] = 0;
    queue.push_back(start);
    while let Some(p) = queue.pop_front() {
        if p == end {
            break;
        } else {
            for &n in &graph[p] {
                if dist[p] + 1 < dist[n] {
                    dist[n] = dist[p] + 1;
                    queue.push_back(n)
                }
            }
        }
    }
    return dist[end];
}

fn main() {
    let mut tunnels = HashMap::new();
    let mut flows = Vec::new();
    let mut graph = Vec::new();
    for l in std::io::stdin().lines() {
        let line = l.unwrap();
        let parts = line.split(|c| c == ' ').collect::<Vec<_>>();
        let name = {
            let len = tunnels.len();
            *tunnels.entry(parts[1].to_owned()).or_insert(len)
        };
        let flow = parts[4]
            .split(|c| c == '=' || c == ';')
            .skip(1)
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let next = parts[9..]
            .iter()
            .map(|x| x.trim_end_matches(','))
            .map(|x| {
                let len = tunnels.len();
                *tunnels.entry(x.to_owned()).or_insert(len)
            })
            .collect::<Vec<_>>();
        flows.resize(tunnels.len(), 0);
        graph.resize_with(tunnels.len(), || Vec::new());
        flows[name] = flow;
        graph[name] = next;
    }
    let mut reduced = (0..flows.len())
        .filter(|&i| i == tunnels["AA"] || flows[i] != 0)
        .collect::<Vec<_>>();
    reduced.sort_by_key(|&i| Reverse(flows[i]));
    let flows = reduced.iter().map(|&i| flows[i]).collect::<Vec<_>>();
    let dists = reduced
        .iter()
        .map(|&from| {
            reduced
                .iter()
                .map(|&to| dist_from_to(&graph, from, to))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start = reduced.iter().position(|&i| i == tunnels["AA"]).unwrap();
    println!(
        "Result: {}",
        find_max(
            &flows,
            &dists,
            start,
            30,
            1 << start,
            0,
            flows.iter().sum(),
            0
        )
    );
}
