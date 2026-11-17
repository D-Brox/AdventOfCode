use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Op {
    Not(String),
    And(String, String),
    And1(String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16),
    Val(u16),
    Line(String),
}

fn resolve(line: String, rules: &mut HashMap<String, Op>) -> u16 {
    let expr = rules[&line].clone();
    let val = match expr {
        Op::Not(line) => !resolve(line, rules),
        Op::And(line_a, line_b) => resolve(line_a, rules) & resolve(line_b, rules),
        Op::And1(line) => 1 & resolve(line, rules),
        Op::Or(line_a, line_b) => resolve(line_a, rules) | resolve(line_b, rules),
        Op::LShift(line, b) => resolve(line, rules) << b,
        Op::RShift(line, b) => resolve(line, rules) >> b,
        Op::Val(v) => v,
        Op::Line(line) => resolve(line, rules),
    };
    rules.insert(line, Op::Val(val));
    val
}

fn parse_rules(input: Vec<String>) -> HashMap<String, Op> {
    input
        .iter()
        .map(|l| {
            let [expr, result] = l.splitn(2, " -> ").collect_array().unwrap();
            let expr = expr.split(' ').collect_vec();
            match expr.len() {
                1 => {
                    let op = expr[0]
                        .parse()
                        .map(Op::Val)
                        .unwrap_or(Op::Line(expr[0].into()));
                    (result.into(), op)
                }
                2 => (result.into(), Op::Not(expr[1].into())),
                3 if expr[0] == "1" => (result.into(), Op::And1(expr[2].into())),
                3 if expr[1] == "AND" => (result.into(), Op::And(expr[0].into(), expr[2].into())),
                3 if expr[1] == "OR" => (result.into(), Op::Or(expr[0].into(), expr[2].into())),
                3 if expr[1] == "RSHIFT" => (
                    result.into(),
                    Op::RShift(expr[0].into(), expr[2].parse().unwrap()),
                ),
                3 if expr[1] == "LSHIFT" => (
                    result.into(),
                    Op::LShift(expr[0].into(), expr[2].parse().unwrap()),
                ),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part1(input: Vec<String>) -> u16 {
    let mut rules = parse_rules(input);
    resolve("a".into(), &mut rules)
}

pub fn part2(input: Vec<String>) -> u16 {
    let mut rules = parse_rules(input);
    let b = resolve("a".into(), &mut rules.clone());
    rules.insert("b".into(), Op::Val(b));
    resolve("a".into(), &mut rules)
}
