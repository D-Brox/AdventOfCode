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

pub fn part1(input: &[String]) -> usize {
    let mut line = input.iter().next().unwrap().to_owned();
    (0..40).for_each(|_| line = look_n_say(&line));
    line.len()
}

pub fn part2(input: &[String]) -> usize {
    let mut line = input.iter().next().unwrap().to_owned();
    (0..50).for_each(|_| line = look_n_say(&line));
    line.len()
}
