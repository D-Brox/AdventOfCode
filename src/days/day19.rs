use std::{collections::HashSet, vec};

use itertools::Itertools;

use crate::days::day19::Symbol::{Bin, NonTerm, Start, Term};

fn parse(input: &mut [String]) -> (Vec<(&str, &str)>, String) {
    for l in input.iter_mut() {
        *l = l
            .replace("Al", "A")
            .replace("Ca", "C")
            .replace("Mg", "M")
            .replace("Si", "S")
            .replace("Th", "T")
            .replace("Ti", "I")
            .replace("Rn", "N")
            .replace("Ar", "R");
    }
    let mol = input.last().unwrap();
    let rules = input[..input.len() - 1]
        .iter()
        .filter_map(|l| l.split_once(" => "))
        .collect_vec();
    (rules, mol.to_owned())
}

pub fn part1(input: &mut [String]) -> usize {
    let (rules, mol) = parse(input);
    let mut set = HashSet::new();
    for (orig, new) in rules {
        let len = orig.len();
        let matches = mol.match_indices(orig);
        for (idx, _) in matches {
            let mut new_mol = mol.clone();
            new_mol.replace_range(idx..(idx + len), new);
            set.insert(new_mol);
        }
    }
    set.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Symbol {
    Start,
    NonTerm(char),
    Term(char),
    Bin(char, usize, usize),
}

// https://en.wikipedia.org/wiki/Chomsky_normal_form
fn cnf(rules: Vec<(&str, &str)>) -> Vec<(Symbol, Vec<Symbol>)> {
    let rules = rules
        .into_iter()
        .map(|(l, r)| {
            let left = l
                .chars()
                .map(|c| match c {
                    'e' => Symbol::Start,
                    _ => Symbol::NonTerm(c),
                })
                .next()
                .unwrap();
            let right = r
                .chars()
                .map(|c| match c {
                    'N' | 'R' | 'Y' => Symbol::Term(c),
                    _ => Symbol::NonTerm(c),
                })
                .collect_vec();
            (left, right)
        })
        .collect_vec();
    // no start rules

    // I can just skip NonTerm(c) -> Term(c) because obviously
    let term_rules = rules
        .into_iter()
        .map(|(l, r)| {
            let r: Vec<Symbol> = r
                .into_iter()
                .map(|s| if let Term(c) = s { NonTerm(c) } else { s })
                .collect();
            (l, r)
        })
        .collect_vec();
    // And I don't count the binarizations in the cyk because they are just one rule lol
    let bin_rules = term_rules
        .into_iter()
        .enumerate()
        .flat_map(|(i, (l, r))| {
            if r.len() <= 2 {
                return vec![(l, r)];
            }
            let NonTerm(c) = l else { unreachable!() };
            let rlen = r.len();
            [(NonTerm(c), vec![r[0], Bin(c, i, 1)])]
                .into_iter()
                .chain(
                    (1..rlen - 2)
                        .into_iter()
                        .map(|n| (Bin(c, i, n), vec![r[n], Bin(c, i, n + 1)])),
                )
                .chain([(Bin(c, i, rlen - 2), vec![r[rlen - 2], r[rlen - 1]])])
                .collect_vec()
        })
        .collect_vec();
    // no del or unit rules

    bin_rules
}

fn cyk(seq: &str, grammar: &[(Symbol, Vec<Symbol>)]) -> usize {
    let symbols = [Start]
        .into_iter()
        .chain(grammar.iter().map(|(l, _)| *l))
        .chain(seq.chars().sorted().dedup().map(NonTerm))
        .sorted()
        .dedup()
        .collect_vec();
    let n = seq.len();
    let mut cost = (0..n)
        .into_iter()
        .map(|i| vec![vec![usize::MAX; symbols.len()]; n - i])
        .collect_vec();

    for (s, cost) in cost[0].iter_mut().enumerate() {
        // terminal production A -> a
        for (v, _) in symbols
            .iter()
            .enumerate()
            .filter(|(_, sy)| *sy == &NonTerm(seq.chars().nth(s).unwrap()))
        {
            cost[v] = 0;
        }
    }
    // Total length of the rule reach, start of reached sequence, and partition of length of the reach
    // Drawing helped understand it lmao
    // See https://en.wikipedia.org/wiki/CYK_algorithm#Example but draw it upsidedown and 0-indexed
    for len in 2..=n {
        for start in 0..=(n - len) {
            for part in 1..len {
                // binary rules A -> B C
                for (l, r) in grammar {
                    let [rb, rc] = r[0..2] else { unreachable!() };
                    let a = symbols.iter().position(|s| s == l).unwrap();
                    let b = symbols.iter().position(|&s| s == rb).unwrap();
                    let c = symbols.iter().position(|&s| s == rc).unwrap();
                    // B . C
                    // . .
                    // A
                    let above = cost[part - 1][start][b];
                    let diag = cost[len - part - 1][start + part][c];
                    if above == usize::MAX || diag == usize::MAX {
                        continue;
                    }
                    cost[len - 1][start][a] = above + diag + usize::from(!matches!(l, Bin(..)));
                }
            }
        }
    }
    cost[n - 1][0][0]
}

pub fn part2(input: &mut [String]) -> usize {
    let (grammar, seq) = parse(input);
    cyk(&seq, &cnf(grammar))
}
