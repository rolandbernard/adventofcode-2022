fn main() {
    let map = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut visible = vec![vec![false; map[0].len()]; map.len()];
    fn test_along<I: Iterator<Item = (usize, usize)>>(
        iter: I,
        map: &Vec<Vec<i32>>,
        visible: &mut Vec<Vec<bool>>,
    ) {
        let mut max = -1;
        for (row, col) in iter {
            if max < map[row][col] {
                visible[row][col] = true;
                max = map[row][col];
            }
        }
    }
    for row in 0..map.len() {
        test_along(
            (0..map[row].len()).map(|col| (row, col)),
            &map,
            &mut visible,
        );
        test_along(
            (0..map[row].len()).rev().map(|col| (row, col)),
            &map,
            &mut visible,
        );
    }
    for col in 0..map[0].len() {
        test_along((0..map.len()).map(|row| (row, col)), &map, &mut visible);
        test_along(
            (0..map.len()).rev().map(|row| (row, col)),
            &map,
            &mut visible,
        );
    }
    let visible_count = visible.iter().flatten().filter(|x| **x).count();
    println!("Result: {visible_count}");
}
