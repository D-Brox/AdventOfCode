use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part1(input: &[String]) -> u32 {
    let mut places = HashSet::new();
    let paths = input
        .iter()
        .map(|l| {
            let mut split = l.split(' ');
            let start = split.next().unwrap();
            places.insert(start);
            split.next();
            let end = split.next().unwrap();
            places.insert(end);
            split.next();
            let dist = split.next().unwrap().parse::<u32>().unwrap();
            vec![((start, end), dist), ((end, start), dist)]
        })
        .concat()
        .into_iter()
        .sorted()
        .collect_vec();
    let paths: HashMap<(&str, &str), u32> = HashMap::from_iter(paths);

    let mut len = u32::MAX;
    for path in places.iter().permutations(places.len()) {
        let start = *path[0];
        len = len.min(
            path[1..]
                .iter()
                .fold((0, start), |(acc, prev), &&curr| {
                    (acc + paths[&(prev, curr)], curr)
                })
                .0,
        );
    }
    len
}

pub fn part2(input: &[String]) -> u32 {
    let mut places = HashSet::new();
    let paths = input
        .iter()
        .map(|l| {
            let mut split = l.split(' ');
            let start = split.next().unwrap();
            places.insert(start);
            split.next();
            let end = split.next().unwrap();
            places.insert(end);
            split.next();
            let dist = split.next().unwrap().parse::<u32>().unwrap();
            vec![((start, end), dist), ((end, start), dist)]
        })
        .concat()
        .into_iter()
        .sorted()
        .collect_vec();
    let paths: HashMap<(&str, &str), u32> = HashMap::from_iter(paths);

    let mut len = u32::MIN;
    for path in places.iter().permutations(places.len()) {
        let start = *path[0];
        len = len.max(
            path[1..]
                .iter()
                .fold((0, start), |(acc, prev), &&curr| {
                    (acc + paths[&(prev, curr)], curr)
                })
                .0,
        );
    }
    len
}
