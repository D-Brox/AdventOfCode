pub fn part1(input: &[String]) -> u32 {
    let line = input.first().unwrap();
    line.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => unreachable!(),
    })
}

pub fn part2(input: &[String]) -> usize {
    let line = input.first().unwrap();
    line.chars()
        .enumerate()
        .fold((0, 0), |acc, (n, c)| {
            if acc.0 < 0 {
                acc
            } else {
                match c {
                    '(' => (acc.0 + 1, n),
                    ')' => (acc.0 - 1, n),
                    _ => unreachable!(),
                }
            }
        })
        .1
        + 1
}
