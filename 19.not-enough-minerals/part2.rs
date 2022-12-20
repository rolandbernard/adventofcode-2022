fn max_geodes(blue: &[[i64; 4]], rob: [i64; 4], res: [i64; 4], time: i64, bound: i64) -> i64 {
    if res[3]
        + rob[3] * time
        + time
            * time.min(
                (res[2]
                    + rob[2] * time
                    + time * time.min((res[1] + rob[1] * time + time * time / 2) / blue[2][1]) / 2)
                    / blue[3][2],
            )
            / 2
        <= bound
    {
        return bound;
    } else {
        let mut max = bound.max(res[3] + rob[3] * time);
        for i in (0..4).rev() {
            if i == 3 || res[i] + time * rob[i] < time * blue.iter().map(|x| x[i]).max().unwrap() {
                let mut skip = 0;
                for j in 0..4 {
                    if blue[i][j] != 0 {
                        if rob[j] == 0 {
                            skip = time;
                        } else {
                            let this = ((blue[i][j] - res[j]).max(0) + rob[j] - 1) / rob[j];
                            if this > skip {
                                skip = this;
                            }
                        }
                    }
                }
                if skip >= time {
                    continue;
                }
                let mut new_res = res;
                for j in 0..4 {
                    new_res[j] += rob[j] * (skip + 1);
                    new_res[j] -= blue[i][j];
                }
                let mut new_rob = rob;
                new_rob[i] += 1;
                max = max_geodes(blue, new_rob, new_res, time - 1 - skip, max);
            }
        }
        return max;
    }
}

fn main() {
    let mut blueprint = Vec::new();
    for line in std::io::stdin().lines().take(3) {
        let line = line.unwrap();
        let split = line
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(". ")
            .map(|t| {
                t.split("costs ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .split(" and ")
                    .map(|l| {
                        let parts = l.split(' ').collect::<Vec<_>>();
                        (
                            parts[0].parse::<i64>().unwrap(),
                            match parts[1].trim_matches('.') {
                                "ore" => 0,
                                "clay" => 1,
                                "obsidian" => 2,
                                "geode" => 3,
                                _ => panic!(),
                            },
                        )
                    })
                    .fold([0; 4], |mut x, (c, i)| {
                        x[i] = c;
                        x
                    })
            })
            .collect::<Vec<_>>();
        blueprint.push(split);
    }
    let quality = blueprint
        .into_iter()
        .map(|b| max_geodes(&b, [1, 0, 0, 0], [0; 4], 32, 0))
        .product::<i64>();
    println!("Result: {quality:?}");
}
