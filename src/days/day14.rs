use std::collections::HashMap;

use itertools::Itertools;

const TIME: u32 = 2503;

pub fn parse(input: &[String]) -> Vec<(&str, u32, u32, u32)> {
    input
        .iter()
        .map(|l| {
            let (name, res) = l.split_once(" can fly ").unwrap();
            let (speed, res) = res.split_once(" km/s for ").unwrap();
            let speed = speed.parse::<u32>().unwrap();
            let (time, res) = res.split_once(" seconds, but then must rest for ").unwrap();
            let time = time.parse::<u32>().unwrap();
            let rest = res.split_once(' ').unwrap().0;
            let rest = rest.parse::<u32>().unwrap();
            (name, speed, time, rest)
        })
        .collect_vec()
}

pub fn part1(input: &[String]) -> u32 {
    let rules = parse(input);
    rules
        .into_iter()
        .map(|(_, s, t, r)| s * t * (TIME / (t + r)) + s * t.min(TIME % (t + r)))
        .max()
        .unwrap()
}

pub fn part2(input: &[String]) -> u32 {
    let rules = parse(input);
    let mut score = rules
        .iter()
        .map(|(n, ..)| (n, 0u32))
        .collect::<HashMap<_, _>>();
    let mut dist = score.clone();
    for i in 0..TIME {
        for (n, s, t, r) in &rules {
            if i % (t + r) < *t {
                *dist.get_mut(&n).unwrap() += s;
            }
        }
        let burst = dist.iter().max_set_by_key(|(_, v)| **v);
        for (n, _) in burst {
            *score.get_mut(n).unwrap() += 1;
        }
    }
    *score.values().max().unwrap()
}
