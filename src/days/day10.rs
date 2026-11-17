use itertools::Itertools;

fn look_n_say(line: &str) -> String {
    let first = line.chars().next().unwrap();
    line.chars()
        .fold(vec![(first, 0)], |mut acc: Vec<(char, usize)>, c| {
            if let Some(last) = acc.last_mut()
                && last.0 == c
            {
                last.1 += 1;
            } else {
                acc.push((c, 1));
            }
            acc
        })
        .iter()
        .map(|(c, n)| format!("{n}{c}"))
        .join("")
}

pub fn part1(input: Vec<String>) -> usize {
    let mut line = input.into_iter().next().unwrap();
    (0..40).for_each(|_| line = look_n_say(&line));
    line.len()
}

pub fn part2(input: Vec<String>) -> usize {
    let mut line = input.into_iter().next().unwrap();
    (0..50).for_each(|_| line = look_n_say(&line));
    line.len()
}
