use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part1(input: Vec<String>) -> i64 {
    let pairs: Vec<(i64, i64)> = input
        .iter()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let mut max = 0;
    for pair in pairs.iter().combinations(2) {
        let [&p1, &p2] = pair[0..2] else {
            unreachable!()
        };
        max = max.max(((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1));
    }
    max
}

pub fn part2(input: Vec<String>) -> usize {
    let red: Vec<(usize, usize)> = input
        .iter()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    // Coordinate compression
    let map_x = red
        .iter()
        .map(|&(x, _)| x)
        .dedup()
        .sorted()
        .enumerate()
        .map(|(i, n)| (n, i + 1))
        .collect::<HashMap<_, _>>();
    let rev_x = map_x.iter().map(|(k, v)| (v, k)).collect::<HashMap<_, _>>();
    let map_y = red
        .iter()
        .map(|&(_, y)| y)
        .dedup()
        .sorted()
        .enumerate()
        .map(|(i, n)| (n, i + 1))
        .collect::<HashMap<_, _>>();
    let rev_y = map_y.iter().map(|(k, v)| (v, k)).collect::<HashMap<_, _>>();
    let mapped_red = red.iter().map(|(x, y)| (map_x[x], map_y[y])).collect_vec();

    let mut max_i = 0;
    let mut max_j = 0;
    let mut color = HashSet::new();
    // Fill border
    for (k, (x, y)) in mapped_red.iter().enumerate() {
        let prev = mapped_red[(k + 1) % mapped_red.len()];
        if prev.0 == *x {
            let pair = [y, &prev.1];
            let [y1, y2] = pair.iter().sorted().collect_array().unwrap();
            max_i = max_i.max(*x);
            max_j = max_j.max(**y2);
            (**y1..=**y2).for_each(|y| {
                color.insert((*x, y));
            });
        } else {
            let pair = [x, &prev.0];
            let [x1, x2] = pair.iter().sorted().collect_array().unwrap();
            max_i = max_i.max(**x2);
            max_j = max_j.max(*y);
            (**x1..=**x2).for_each(|x| {
                color.insert((x, *y));
            });
        }
    }

    // Fill the inside
    for j in 1..=max_j {
        let mut inside = false;
        let mut up = false;
        for i in 1..=max_i {
            if mapped_red.contains(&(i, j)) {
                if !inside {
                    up = color.contains(&(i, j - 1));
                    inside = true;
                } else if (color.contains(&(i, j - 1)) && up && !color.contains(&(i, j + 1)))
                    || (!up && !color.contains(&(i, j - 1)))
                {
                    inside = false;
                }
            } else if color.contains(&(i, j)) {
                inside = !inside;
            } else if inside {
                color.insert((i, j));
            }
        }
    }

    // Check all
    let mut max = 0;
    for pair in mapped_red.iter().combinations(2) {
        let [p1, p2] = pair[0..2] else { unreachable!() };
        let pair = [p1.0, p2.0];
        let [x1, x2] = pair.iter().sorted().collect_array().unwrap();
        let pair = [p1.1, p2.1];
        let [y1, y2] = pair.iter().sorted().collect_array().unwrap();
        if (*x1..=*x2)
            .cartesian_product(*y1..=*y2)
            .all(|(x, y)| color.contains(&(x, y)))
        {
            let x1 = rev_x[x1];
            let x2 = rev_x[x2];
            let y1 = rev_y[y1];
            let y2 = rev_y[y2];
            max = max.max((y2 - y1 + 1) * (x2 - x1 + 1));
        }
    }
    max
}
