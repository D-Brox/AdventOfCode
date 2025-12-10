use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::prelude::*;
use z3::{Optimize, SatResult, ast::Int};

pub fn part1(input: Vec<String>) -> usize {
    let indicators: Vec<HashSet<usize>> = input
        .iter()
        .map(|l| {
            l.split_once(' ')
                .unwrap()
                .0
                .trim_matches(['[', ']'])
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if matches!(c, '#') { Some(i) } else { None })
                .collect()
        })
        .collect();
    let buttons = input
        .iter()
        .map(|l| {
            l.split(' ')
                .filter(|s| !s.contains(['[', '{']))
                .map(|s| {
                    s.trim_matches(['(', ')'])
                        .split(',')
                        .flat_map(|s| s.parse::<usize>())
                        .collect::<Vec<_>>()
                })
                .collect_vec()
        })
        .collect_vec();
    let mut sum = 0;
    for (i, bset) in buttons.iter().enumerate() {
        let truth = &indicators[i];
        for pset in bset.iter().powerset() {
            if pset.is_empty() {
                continue;
            }
            let mut counts = HashMap::new();
            pset.iter().for_each(|set| {
                set.iter()
                    .for_each(|n| *counts.entry(n).or_insert(false) ^= true)
            });
            let set = counts
                .into_iter()
                .filter_map(|(k, v)| if v { Some(*k) } else { None })
                .collect::<HashSet<_>>();
            if truth == &set {
                sum += pset.len();
                break;
            }
        }
    }
    sum
}

pub fn part2(input: Vec<String>) -> u64 {
    let joltage: Vec<Vec<u64>> = input
        .iter()
        .map(|l| {
            l.rsplit_once(' ')
                .unwrap()
                .1
                .trim_matches(['{', '}'])
                .split(',')
                .flat_map(|s| s.parse())
                .collect()
        })
        .collect();
    let buttons = input
        .iter()
        .map(|l| {
            l.split(' ')
                .filter(|s| !s.contains(['[', '{']))
                .map(|s| {
                    s.trim_matches(['(', ')'])
                        .split(',')
                        .flat_map(|s| s.parse::<usize>())
                        .collect::<Vec<_>>()
                })
                .collect_vec()
        })
        .collect_vec();
    joltage
        .par_iter()
        .zip(buttons)
        .map(|(jolt, but)| {
            let total = Int::fresh_const("total");
            let opt = Optimize::new();
            let presses = but
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let b = Int::fresh_const(&format!("b{i}"));
                    opt.assert(&b.ge(0));
                    b
                })
                .collect_vec();
            opt.assert(&total.eq(Int::add(&presses)));

            jolt.iter().enumerate().for_each(|(j, jl)| {
                let sum = Int::add(
                    &but.iter()
                        .enumerate()
                        .filter_map(|(i, b)| {
                            if b.contains(&j) {
                                Some(&presses[i])
                            } else {
                                None
                            }
                        })
                        .collect_vec(),
                );
                opt.assert(&sum.eq(Int::from_u64(*jl)));
            });
            opt.minimize(&total);
            if let SatResult::Sat = opt.check(&[]) {
                opt.get_model()
                    .unwrap()
                    .eval(&total, true)
                    .unwrap()
                    .as_u64()
                    .unwrap()
            } else {
                panic!("Failed to solve");
            }
        })
        .sum()
}
