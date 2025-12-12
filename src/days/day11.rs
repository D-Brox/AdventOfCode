use std::collections::HashMap;

use itertools::Itertools;

fn search_path<'a>(
    counts: &mut HashMap<&'a str, usize>,
    map: &HashMap<&str, Vec<&'a str>>,
    label: &'a str,
) -> usize {
    if counts.contains_key(label) {
        return counts[label];
    }
    let count = map
        .get(label)
        .map(|l| {
            l.iter()
                .map(|next| search_path(counts, map, next))
                .sum::<usize>()
        })
        .unwrap_or_default();
    counts.insert(label, count);
    count
}

pub fn part1(input: Vec<String>) -> usize {
    let map = input
        .iter()
        .map(|l| {
            let labels = l.split_whitespace().collect_vec();
            let start = labels[0].trim_end_matches(':');
            let ends = labels[1..].iter().map(<&str>::to_owned).collect_vec();
            (start, ends)
        })
        .collect::<HashMap<_, _>>();

    let mut counts = HashMap::from([("out", 1)]);
    search_path(&mut counts, &map, "you")
}

pub fn part2(input: Vec<String>) -> usize {
    let map = input
        .iter()
        .map(|l| {
            let labels = l.split_whitespace().collect_vec();
            let start = labels[0].trim_end_matches(':');
            let ends = labels[1..].iter().map(<&str>::to_owned).collect_vec();
            (start, ends)
        })
        .collect::<HashMap<_, _>>();

    let mut count = 0;
    let mut fft_dac = HashMap::from([("dac", 1)]);
    let count_fft_dac = search_path(&mut fft_dac, &map, "fft");
    if count_fft_dac != 0 {
        let mut svr_fft = HashMap::from([("fft", 1)]);
        let mut dac_out = HashMap::from([("out", 1)]);
        count += search_path(&mut svr_fft, &map, "svr")
            * count_fft_dac
            * search_path(&mut dac_out, &map, "dac");
    }

    let mut dac_fft = HashMap::from([("fft", 1)]);
    let count_fft_dac = search_path(&mut dac_fft, &map, "dac");
    if count_fft_dac != 0 {
        let mut svr_dac = HashMap::from([("dac", 1)]);
        let mut fft_out = HashMap::from([("out", 1)]);
        count += search_path(&mut svr_dac, &map, "svr")
            * count_fft_dac
            * search_path(&mut fft_out, &map, "fft");
    }
    count
}
