use itertools::Itertools;

fn parse(input: &[String]) -> Vec<u32> {
    input.iter().map(|l| l.parse().unwrap()).collect_vec()
}

pub fn part1(input: &[String]) -> usize {
    let containers = parse(input);
    (0..containers.len())
        .powerset()
        .filter(|set| set.iter().map(|&n| containers[n]).sum::<u32>() == 150)
        .count()
}

pub fn part2(input: &[String]) -> usize {
    let containers = parse(input);
    (0..containers.len())
        .powerset()
        .filter(|set| set.iter().map(|&n| containers[n]).sum::<u32>() == 150)
        .min_set_by_key(Vec::len)
        .len()
}
