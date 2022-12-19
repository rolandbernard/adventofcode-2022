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
    } else if time == 0 {
        return res[3];
    } else {
        let mut max = bound;
        for i in (0..4).rev() {
            if blue[i][0] <= res[0]
                && blue[i][1] <= res[1]
                && blue[i][2] <= res[2]
                && blue[i][3] <= res[3]
            {
                let mut new_res = res;
                for j in 0..4 {
                    new_res[j] -= blue[i][j];
                    new_res[j] += rob[j];
                }
                let mut new_rob = rob;
                new_rob[i] += 1;
                max = max_geodes(blue, new_rob, new_res, time - 1, max);
            }
        }
        let mut new_res = res;
        for j in 0..4 {
            new_res[j] += rob[j];
        }
        return max_geodes(blue, rob, new_res, time - 1, max);
    }
}

fn main() {
    let mut blueprint = Vec::new();
    for line in std::io::stdin().lines() {
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
        .enumerate()
        .map(|(i, b)| (i as i64 + 1) * max_geodes(&b, [1, 0, 0, 0], [0; 4], 24, 0))
        .sum::<i64>();
    println!("Result: {quality:?}");
}
