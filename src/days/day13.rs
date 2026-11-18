use std::collections::HashMap;

use itertools::Itertools;

pub fn parse(input: &[String]) -> (Vec<&str>, HashMap<(&str, &str), i32>) {
    let lines = input
        .iter()
        .map(|l| {
            let (name, res) = l.split_once(" would ").unwrap();
            let (gain, res) = res.split_once(" ").unwrap();
            let gain = gain == "gain";
            let (val, res) = res
                .split_once(" happiness units by sitting next to ")
                .unwrap();
            let val = val.parse::<i32>().unwrap();
            let val = if gain { val } else { -val };
            let res = res.strip_suffix('.').unwrap();
            ((name, res), val)
        })
        .collect_vec();
    let names: Vec<&str> = lines
        .iter()
        .map(|((n, _), _)| *n)
        .sorted()
        .dedup()
        .collect();
    let rules: HashMap<(&str, &str), i32> = HashMap::from_iter(lines);
    (names, rules)
}

fn opt(start: &str, names: Vec<&str>, rules: HashMap<(&str, &str), i32>) -> i32 {
    let total = names.len();
    names
        .into_iter()
        .permutations(total)
        .map(|mut order| {
            order.push(start);
            let first = *order.first().unwrap();
            order.iter().tuple_windows().fold(
                rules.get(&(start, first)).unwrap_or(&0) + rules.get(&(first, start)).unwrap_or(&0),
                |acc, (&l, &r)| {
                    acc + rules.get(&(l, r)).unwrap_or(&0) + rules.get(&(r, l)).unwrap_or(&0)
                },
            )
        })
        .max()
        .unwrap()
}

pub fn part1(input: Vec<String>) -> i32 {
    let (mut names, rules) = parse(&input);
    opt(names.pop().unwrap(), names, rules)
}

pub fn part2(input: Vec<String>) -> i32 {
    let (names, rules) = parse(&input);
    opt("", names, rules)
}
