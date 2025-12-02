use itertools::Itertools;
use number_theory::{Factorization, NumberTheory};
use std::collections::HashSet;

pub fn solution1(input: Vec<String>) -> u64 {
    let ranges = input[0]
        .split(',')
        .map(|r| r.split('-').collect_tuple::<(&str, &str)>().unwrap())
        .collect_vec();
    let mut sum = 0;

    for pair in ranges {
        let lower;
        let llen;
        let upper;
        let ulen;
        match (pair.0.len() % 2, pair.1.len() % 2) {
            (0, 0) => {
                llen = pair.0.len();
                lower = pair.0.parse::<u64>().unwrap();
                ulen = pair.1.len();
                upper = pair.1.parse::<u64>().unwrap();
            }
            (1, 0) => {
                llen = pair.0.len() + 1;
                lower = 10u64.pow(llen as u32 - 1);
                ulen = pair.1.len();
                upper = pair.1.parse::<u64>().unwrap();
            }
            (0, 1) => {
                llen = pair.0.len();
                lower = pair.0.parse::<u64>().unwrap();
                ulen = pair.1.len() - 1;
                upper = 10u64.pow(ulen as u32) - 1;
            }
            (1, 1) => continue,
            _ => unreachable!("0 <= n mod 2 < 2"),
        }

        if llen == ulen {
            let start = lower / (10u64.pow(llen as u32 / 2));
            let end = upper / (10u64.pow(ulen as u32 / 2));
            for v in start..=end {
                let val = v * (10u64.pow(llen as u32 / 2) + 1);
                if (lower..=upper).contains(&val) {
                    sum += val;
                }
            }
        } else {
            {
                let start = lower / (10u64.pow(llen as u32 / 2));
                let end = 10u64.pow(llen as u32 / 2) - 1;
                for v in start..=end {
                    let val = v * (10u64.pow(llen as u32 / 2) + 1);
                    if (lower..(10u64.pow(llen as u32))).contains(&val) {
                        sum += val;
                    }
                }
            }
            {
                let start = 10u64.pow(ulen as u32 / 2);
                let end = upper / (10u64.pow(ulen as u32 / 2));
                for v in start..=end {
                    let val = v * (10u64.pow(llen as u32 / 2 + 1) + 1);
                    if (10u64.pow(llen as u32)..=upper).contains(&val) {
                        sum += val;
                    }
                }
            }
        }
    }

    sum
}

pub fn solution2(input: Vec<String>) -> u64 {
    let ranges = input[0]
        .split(',')
        .map(|r| r.split('-').collect_tuple::<(&str, &str)>().unwrap())
        .collect_vec();
    let mut repeaters = HashSet::new();
    for pair in ranges {
        let lower = pair.0.parse::<u64>().unwrap();
        let llen = pair.0.len();
        let upper = pair.1.parse::<u64>().unwrap();
        let ulen = pair.1.len();
        for i in llen..=ulen {
            let reps = get_repeaters(i);
            let rep = reps
                .iter()
                .filter(|rep| (lower..=upper).contains(&rep))
                .collect_vec();
            repeaters.extend(rep);
        }
    }
    repeaters.iter().sum()
}

fn get_repeaters(length: usize) -> Vec<u64> {
    let mut repeaters = vec![];
    let divisors = length.divisors();
    for div in divisors {
        if div == length {
            continue;
        }
        let times = length / div;
        let mut repeater = 0;
        for i in 0..times {
            repeater += 10u64.pow((i * div) as u32);
        }
        repeaters.append(
            &mut (10u64.pow(div as u32 - 1)..10u64.pow(div as u32))
                .map(|i| i * repeater)
                .collect_vec(),
        );
    }
    repeaters
}

trait Divisors {
    fn divisors(&self) -> Vec<Self>
    where
        Self: Sized;
}

impl Divisors for usize {
    fn divisors(&self) -> Vec<Self> {
        let factors = self.factor().unwrap_or({
            let mut f = Factorization::new();
            f.add_factor(1);
            f
        });
        factors
            .pair_iter()
            .flat_map(|(b, p)| [b].repeat(*p as usize))
            .powerset()
            .map(|factors| factors.into_iter().product())
            .dedup()
            .collect_vec()
    }
}
