use itertools::Itertools;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const NAUGHTY: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

fn is_nice1(s: &str) -> bool {
    let mut vowels = if matches!(s.chars().next(), Some(x) if VOWELS.contains(&x)) {
        1
    } else {
        0
    };
    let mut repeat = false;
    let mut naughty = false;
    s.chars().tuple_windows().for_each(|(a, b)| {
        repeat |= a == b;
        naughty |= NAUGHTY.contains(&(a, b));
        if VOWELS.contains(&b) {
            vowels += 1;
        }
    });
    !naughty && vowels >= 3 && repeat
}

pub fn part1(input: Vec<String>) -> usize {
    input.into_iter().filter(|s| is_nice1(s)).count()
}

fn is_nice2(s: &str) -> bool {
    let chars = s.chars().collect_vec();
    let mut pair = false;
    let mut repeat = false;
    for i in 0..(chars.len() - 2) {
        chars[i + 2..]
            .iter()
            .tuple_windows()
            .for_each(|p: (&char, &char)| pair |= p == (&chars[i], &chars[i + 1]));
        repeat |= chars[i] == chars[i + 2];
    }
    pair && repeat
}

pub fn part2(input: Vec<String>) -> usize {
    input.into_iter().filter(|s| is_nice2(s)).count()
}
