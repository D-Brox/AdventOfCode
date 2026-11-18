use std::{collections::HashMap, sync::LazyLock};

use itertools::Itertools;

const TAPE: LazyLock<HashMap<&str, i32>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ])
});

pub fn part1(input: &[String]) -> u16 {
    for line in input {
        let (sue, detection) = line.split_once(": ").unwrap();
        let is_sue = detection
            .split([':', ','])
            .map(str::trim)
            .chunks(2)
            .into_iter()
            .all(|mut chunk| {
                let (k, v) = (chunk.next().unwrap(), chunk.next().unwrap());
                let v = v.parse().unwrap();
                TAPE.get(k) == Some(&v)
            });

        if is_sue {
            return sue[4..].parse::<u16>().unwrap();
        }
    }
    todo!()
}

pub fn part2(input: &[String]) -> u16 {
    for line in input {
        let (sue, detection) = line.split_once(": ").unwrap();
        let is_sue = detection
            .split([':', ','])
            .map(str::trim)
            .chunks(2)
            .into_iter()
            .all(|mut chunk| {
                let (k, v) = (chunk.next().unwrap(), chunk.next().unwrap());
                let v = v.parse().unwrap();
                let tape = *TAPE.get(k).unwrap();
                match k {
                    "cats" | "trees" => tape < v,
                    "pomeranians" | "goldfish" => tape > v,
                    _ => tape == v,
                }
            });

        if is_sue {
            return sue[4..].parse::<u16>().unwrap();
        }
    }
    todo!()
}
