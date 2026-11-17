use std::collections::HashSet;

pub fn part1(input: Vec<String>) -> usize {
    let line = input.first().unwrap();
    let mut curr = (0i32, 0i32);
    let mut set = HashSet::from([curr]);
    line.chars().for_each(|c| {
        match c {
            '>' => curr.0 += 1,
            '<' => curr.0 -= 1,
            '^' => curr.1 += 1,
            'v' => curr.1 -= 1,
            _ => unreachable!(),
        }
        set.insert(curr);
    });
    set.len()
}

pub fn part2(input: Vec<String>) -> usize {
    let line = input.first().unwrap();
    let mut santa = (0i32, 0i32);
    let mut robo = (0i32, 0i32);
    let mut set = HashSet::from([santa]);
    line.chars().enumerate().for_each(|(n, c)| {
        let curr = if n % 2 == 0 { &mut santa } else { &mut robo };
        match c {
            '>' => curr.0 += 1,
            '<' => curr.0 -= 1,
            '^' => curr.1 += 1,
            'v' => curr.1 -= 1,
            _ => unreachable!(),
        }
        set.insert(*curr);
    });
    set.len()
}
