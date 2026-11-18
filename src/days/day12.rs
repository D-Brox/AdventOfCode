use itertools::Itertools;

pub fn part1(input: &[String]) -> i32 {
    input
        .iter()
        .next()
        .map(|l| {
            l.split(['{', '}', '[', ']', ',', ':'])
                .map(|n| n.parse::<i32>().unwrap_or(0))
                .sum()
        })
        .unwrap()
}

pub fn part2(input: &[String]) -> i32 {
    let mut line = input.iter().next().unwrap().to_owned();
    while let Some((left, right)) = line.split_once(r#":"red""#) {
        let mut open = 0;
        let l = left
            .chars()
            .collect_vec()
            .iter()
            .rposition(|c| {
                if *c == '}' {
                    open += 1;
                    false
                } else if *c != '{' {
                    false
                } else if open != 0 {
                    open -= 1;
                    false
                } else {
                    true
                }
            })
            .unwrap();
        open = 0;
        let r = right
            .chars()
            .collect_vec()
            .iter()
            .position(|c| {
                if *c == '{' {
                    open += 1;
                    false
                } else if *c != '}' {
                    false
                } else if open != 0 {
                    open -= 1;
                    false
                } else {
                    true
                }
            })
            .unwrap();
        line = String::new() + &left[..=l] + &right[r..];
    }
    part1(&[line])
}
