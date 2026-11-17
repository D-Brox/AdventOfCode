use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    On,
    Off,
    Toggle,
}

fn split_pair(s: &str) -> (usize, usize) {
    s.split(',')
        .map(|p| p.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn part1(input: Vec<String>) -> usize {
    let lines = input
        .iter()
        .map(|l| {
            let mut l = l.split(' ');
            let i = if let Some("toggle") = l.next() {
                Instruction::Toggle
            } else if let Some("on") = l.next() {
                Instruction::On
            } else {
                Instruction::Off
            };
            let s = split_pair(l.next().unwrap());
            l.next();
            let e = split_pair(l.next().unwrap());
            (i, s, e)
        })
        .collect_vec();

    let mut lights = [[false; 1000]; 1000];

    for (i, (s1, s2), (e1, e2)) in lines {
        let light = (s1..=e1).cartesian_product(s2..=e2);
        match i {
            Instruction::On => light.for_each(|(i, j)| lights[i][j] = true),
            Instruction::Off => light.for_each(|(i, j)| lights[i][j] = false),
            Instruction::Toggle => light.for_each(|(i, j)| lights[i][j].toggle()),
        }
    }

    lights.concat().into_iter().filter(|&a| a).count()
}

pub fn part2(input: Vec<String>) -> u32 {
    let lines = input
        .iter()
        .map(|l| {
            let mut l = l.split(' ');
            let i = if let Some("toggle") = l.next() {
                Instruction::Toggle
            } else if let Some("on") = l.next() {
                Instruction::On
            } else {
                Instruction::Off
            };
            let s = split_pair(l.next().unwrap());
            l.next();
            let e = split_pair(l.next().unwrap());
            (i, s, e)
        })
        .collect_vec();

    let mut lights = [[0u32; 1000]; 1000];

    for (i, (s1, s2), (e1, e2)) in lines {
        let light = (s1..=e1).cartesian_product(s2..=e2);
        match i {
            Instruction::On => light.for_each(|(i, j)| lights[i][j] += 1),
            Instruction::Off => {
                light.for_each(|(i, j)| lights[i][j] = lights[i][j].saturating_sub(1))
            }
            Instruction::Toggle => light.for_each(|(i, j)| lights[i][j] += 2),
        }
    }

    lights.concat().into_iter().sum()
}
