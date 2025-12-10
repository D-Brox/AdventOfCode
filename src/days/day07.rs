use std::collections::HashSet;

pub fn part1(input: Vec<String>) -> usize {
    let mut splits = 0;
    let mut tachyons = HashSet::new();
    tachyons.insert(input[0].find('S').unwrap());
    for i in input.iter().skip(1) {
        tachyons = tachyons
            .into_iter()
            .flat_map(|t| {
                if i.chars().nth(t).unwrap() == '^' {
                    splits += 1;
                    vec![t - 1, t + 1]
                } else {
                    vec![t]
                }
            })
            .collect();
    }
    splits
}

pub fn part2(input: Vec<String>) -> usize {
    let mut tachyons = vec![0; input[0].len()];
    tachyons[input[0].find('S').unwrap()] = 1;
    for i in 1..input.len() {
        let mut after = vec![0; input[0].len()];
        for (j, (c, t)) in input[i].chars().zip(&tachyons).enumerate() {
            if c == '^' {
                after[j - 1] += t;
                after[j + 1] += t;
            } else {
                after[j] += t;
            }
        }
        std::mem::swap(&mut tachyons, &mut after);
    }
    tachyons.iter().sum()
}
