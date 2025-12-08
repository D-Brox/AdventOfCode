use itertools::Itertools;

pub fn part1(mut input: Vec<String>) -> u64 {
    let ops = input.pop().unwrap();
    let ops = ops.split_whitespace().collect_vec();
    let inputs = input
        .iter()
        .map(|i| {
            i.split_whitespace()
                .flat_map(str::parse::<u64>)
                .collect_vec()
        })
        .collect_vec();

    let mut sum = 0u64;
    for (i, op) in ops.into_iter().enumerate() {
        let line = inputs.iter().map(|l| l[i]);
        match op {
            "+" => sum += line.sum::<u64>(),
            "*" => sum += line.product::<u64>(),
            _ => unreachable!(),
        }
    }
    sum
}

pub fn part2(mut input: Vec<String>) -> u64 {
    input.iter_mut().for_each(|l| l.push(' '));
    let ops = input.pop().unwrap();
    let (ops, sizes): (Vec<_>, Vec<_>) = ops
        .split(' ')
        .fold(vec![], |mut acc, s| {
            if s.is_empty() {
                let last: &mut (_, usize) = acc.last_mut().unwrap();
                last.1 = last.1 + 1;
            } else {
                acc.push((s, 2));
            }
            acc
        })
        .into_iter()
        .unzip();
    let inputs = input
        .iter()
        .map(|i| {
            let mut line = vec![];
            let mut raw = i.chars();
            for &s in &sizes {
                let mut n = String::new();
                for _ in 0usize..s {
                    n.push(raw.next().unwrap_or(' '));
                }
                line.push(n);
            }
            line
        })
        .collect_vec();
    let mut sum = 0u64;
    for (i, op) in ops.into_iter().enumerate() {
        let line = inputs.iter().fold(vec![], |mut acc: Vec<String>, l| {
            for (i, c) in l[i].chars().enumerate() {
                if let Some(n) = acc.get_mut(i) {
                    n.push(c);
                } else {
                    acc.push(String::from(c));
                }
            }
            acc
        });
        let line = line.iter().filter_map(|s| s.trim().parse::<u64>().ok());
        match op {
            "+" => sum += line.sum::<u64>(),
            "*" => sum += line.product::<u64>(),
            _ => unreachable!(),
        }
    }
    sum
}
