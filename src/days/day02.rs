use itertools::Itertools;

pub fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|l| {
            let sizes = l
                .split('x')
                .map(|s| s.parse::<u32>().unwrap())
                .combinations(2)
                .map(|v| {
                    let [a, b] = v[0..2] else { unreachable!() };
                    a * b
                })
                .collect_vec();
            let min = sizes.iter().min().unwrap();
            min + 2 * sizes.iter().sum::<u32>()
        })
        .sum()
}

pub fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|l| {
            let sizes = l
                .split('x')
                .map(|s| s.parse::<u32>().unwrap())
                .sorted()
                .collect_vec();
            2 * (sizes[0] + sizes[1]) + sizes.iter().product::<u32>()
        })
        .sum()
}
