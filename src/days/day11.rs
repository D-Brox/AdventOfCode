use itertools::Itertools;

fn is_valid(array: &[u8]) -> bool {
    let mut triplet = false;
    let mut pairs = vec![];
    array.iter().tuple_windows().fold(26, |a, (&b, &c)| {
        if a + 1 == b && b + 1 == c {
            triplet = true;
        }
        if b == c {
            pairs.push(b);
        }
        b
    });
    pairs.sort_unstable();
    pairs.dedup();
    triplet && pairs.len() >= 2
}

fn next_pass(array: &mut [u8]) -> Vec<u8> {
    for b in array.iter_mut().rev() {
        *b += 1;
        if *b < 26 {
            break;
        }
        *b = 0;
    }
    if let Some(idx) = array.iter().position(|b| matches!(b, 8 | 11 | 14)) {
        let mut array = array.iter().copied().take(idx + 1).collect_vec();
        array.resize(8, 0);
        array[idx] += 1;
        array
    } else {
        array.into()
    }
}

pub fn part1(input: &[String]) -> String {
    let line = input.iter().next().unwrap();
    let mut array = line.chars().map(|c| c as u8 - 97).collect_vec();
    array = next_pass(&mut array);
    loop {
        if is_valid(&array) {
            return array.iter_mut().map(|b| (*b + 97) as char).collect();
        }
        array = next_pass(&mut array);
    }
}

pub fn part2(input: &[String]) -> String {
    part1(&[part1(input)])
}
