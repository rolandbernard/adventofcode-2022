fn main() {
    let map = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut score = vec![vec![1; map[0].len()]; map.len()];
    fn test_along<I: Iterator<Item = (usize, usize)>>(
        iter: I,
        map: &Vec<Vec<usize>>,
        score: &mut Vec<Vec<usize>>,
    ) {
        let mut dist = [0; 10];
        for (row, col) in iter {
            score[row][col] *= dist[map[row][col]];
            for i in 0..map[row][col] + 1 {
                dist[i] = 1;
            }
            for i in map[row][col] + 1..10 {
                dist[i] += 1;
            }
        }
    }
    for row in 0..map.len() {
        test_along((0..map[row].len()).map(|col| (row, col)), &map, &mut score);
        test_along(
            (0..map[row].len()).rev().map(|col| (row, col)),
            &map,
            &mut score,
        );
    }
    for col in 0..map[0].len() {
        test_along((0..map.len()).map(|row| (row, col)), &map, &mut score);
        test_along((0..map.len()).rev().map(|row| (row, col)), &map, &mut score);
    }
    let max_score = score.iter().flatten().max().unwrap();
    println!("Result: {max_score}");
}
