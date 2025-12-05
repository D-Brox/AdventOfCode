use itertools::Itertools;
use rangetools::Rangetools;

pub fn solution1(input: Vec<String>) -> usize {
    let (ranges, items) = input
        .split(String::is_empty)
        .map(|i| i.iter().collect_vec())
        .collect_tuple()
        .unwrap();
    let ranges = ranges
        .iter()
        .map(|range| {
            let (l, r) = range
                .split('-')
                .flat_map(str::parse::<u64>)
                .collect_tuple()
                .unwrap();
            l..=r
        })
        .fold((0..0).union(0..0), |l, r| l.union(r));

    let items = items.iter().flat_map(|s| s.parse::<u64>()).collect_vec();
    items.into_iter().filter(|&i| ranges.contains(i)).count()
}

pub fn solution2(input: Vec<String>) -> usize {
    let ranges = input
        .split(String::is_empty)
        .next()
        .unwrap()
        .iter()
        .map(|range| {
            let (l, r) = range
                .split('-')
                .flat_map(str::parse::<u64>)
                .collect_tuple()
                .unwrap();
            l..=r
        })
        .fold((0..0).union(0..0), |l, r| l.union(r));
    ranges.into_iter().count()
}
