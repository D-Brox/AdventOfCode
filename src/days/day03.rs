use itertools::Itertools;

fn find_idx_max(slice: &str) -> usize {
    slice
        .chars()
        .collect_vec()
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap_or(0)
}

pub fn part1(input: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in input {
        let idx1 = find_idx_max(&line[..(line.len() - 1)]);
        let idx2 = find_idx_max(&line[(idx1 + 1)..]) + idx1 + 1;
        sum += format!(
            "{}{}",
            line.chars().nth(idx1).unwrap(),
            line.chars().nth(idx2).unwrap()
        )
        .parse::<u32>()
        .unwrap();
    }
    sum
}

pub fn part2(input: Vec<String>) -> u64 {
    let mut sum = 0;
    for line in input {
        let mut slice_filtered = line[0..12].to_string();
        let mut start;
        let mut next_start = 0;
        'outer: for c in line[12..line.len()].chars() {
            start = next_start;
            slice_filtered.push(c);
            for i in start..12 {
                let left = slice_filtered.chars().nth(i).unwrap();
                let right = slice_filtered.chars().nth(i + 1).unwrap();
                if left < right {
                    next_start = if i > 0 && slice_filtered.chars().nth(i - 1).unwrap() >= left {
                        i - 1
                    } else {
                        i
                    };
                    slice_filtered.remove(i);
                    continue 'outer;
                }
            }
            next_start = 11;
            slice_filtered.pop();
        }
        sum += slice_filtered[0..12].parse::<u64>().unwrap();
    }
    sum
}
