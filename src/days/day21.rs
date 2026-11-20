use itertools::Itertools;

const WEAPONS: [(&str, i32, i32); 5] = [
    ("Dagger", 8, 4),
    ("Shortsword", 10, 5),
    ("Warhammer", 25, 6),
    ("Longsword", 40, 7),
    ("Greataxe", 74, 8),
];

const ARMOR: [(&str, i32, i32); 5] = [
    ("Leather", 13, 1),
    ("Chainmail", 31, 2),
    ("Splintmail", 53, 3),
    ("Bandedmail", 75, 4),
    ("Platemail", 102, 5),
];

const RINGS: [(&str, i32, i32, i32); 6] = [
    ("Damage +1", 25, 1, 0),
    ("Damage +2", 50, 2, 0),
    ("Damage +3", 100, 3, 0),
    ("Defense +1", 20, 0, 1),
    ("Defense +2", 40, 0, 2),
    ("Defense +3", 80, 0, 3),
];

pub fn part1(input: &[String]) -> i32 {
    let (b_hp, b_atk, b_def) = input
        .iter()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();

    WEAPONS
        .iter()
        .cartesian_product(ARMOR.iter().powerset().take_while(|s| s.len() <= 1))
        .cartesian_product(RINGS.iter().powerset().take_while(|s| s.len() <= 2))
        .filter_map(|((w, a), r)| {
            let mut p_atk = w.2 - b_def;
            let mut b_atk = b_atk;
            let mut cost = w.1;
            for a in &a {
                cost += a.1;
                b_atk -= a.2;
            }
            for r in &r {
                cost += r.1;
                p_atk += r.2;
                b_atk -= r.3;
            }

            p_atk = p_atk.max(1);
            b_atk = b_atk.max(1);

            let turns = b_hp.div_ceil(p_atk) - 1;
            if 100 > turns * b_atk {
                Some(cost)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

pub fn part2(input: &[String]) -> i32 {
    let (b_hp, b_atk, b_def) = input
        .iter()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();

    WEAPONS
        .iter()
        .cartesian_product(ARMOR.iter().powerset().take_while(|s| s.len() <= 1))
        .cartesian_product(RINGS.iter().powerset().take_while(|s| s.len() <= 2))
        .filter_map(|((w, a), r)| {
            let mut p_atk = w.2 - b_def;
            let mut b_atk = b_atk;
            let mut cost = w.1;
            for a in &a {
                cost += a.1;
                b_atk -= a.2;
            }
            for r in &r {
                cost += r.1;
                p_atk += r.2;
                b_atk -= r.3;
            }

            p_atk = p_atk.max(1);
            b_atk = b_atk.max(1);

            let turns = b_hp.div_ceil(p_atk) - 1;
            if 100 <= turns * b_atk {
                Some(cost)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}
