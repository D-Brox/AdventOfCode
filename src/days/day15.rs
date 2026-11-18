use itertools::Itertools;

pub fn parse(input: &[String]) -> Vec<(i32, i32, i32, i32, i32)> {
    input
        .iter()
        .map(|l| {
            let res = l.split_once(": capacity ").unwrap().1;
            let (capacity, res) = res.split_once(", durability ").unwrap();
            let capacity = capacity.parse::<i32>().unwrap();
            let (durability, res) = res.split_once(", flavor ").unwrap();
            let durability = durability.parse::<i32>().unwrap();
            let (flavor, res) = res.split_once(", texture ").unwrap();
            let flavor = flavor.parse::<i32>().unwrap();
            let (texture, calories) = res.split_once(", calories ").unwrap();
            let texture = texture.parse::<i32>().unwrap();
            let calories = calories.parse::<i32>().unwrap();
            (capacity, durability, flavor, texture, calories)
        })
        .collect_vec()
}

pub fn part1(input: &[String]) -> i32 {
    let rules = parse(input);
    let mut max = 0;
    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                let l = 100 - i - j - k;
                let capacity = i * rules[0].0 + j * rules[1].0 + k * rules[2].0 + l * rules[3].0;
                let durability = i * rules[0].1 + j * rules[1].1 + k * rules[2].1 + l * rules[3].1;
                let flavor = i * rules[0].2 + j * rules[1].2 + k * rules[2].2 + l * rules[3].2;
                let texture = i * rules[0].3 + j * rules[1].3 + k * rules[2].3 + l * rules[3].3;
                if [capacity, durability, flavor, texture]
                    .into_iter()
                    .any(i32::is_negative)
                {
                    continue;
                } else {
                    max = max.max(capacity * durability * flavor * texture);
                }
            }
        }
    }
    max
}

pub fn part2(input: &[String]) -> i32 {
    let rules = parse(input);
    let mut max = 0;
    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                let l = 100 - i - j - k;
                let calories = i * rules[0].4 + j * rules[1].4 + k * rules[2].4 + l * rules[3].4;
                if calories != 500 {
                    continue;
                }
                let capacity = i * rules[0].0 + j * rules[1].0 + k * rules[2].0 + l * rules[3].0;
                let durability = i * rules[0].1 + j * rules[1].1 + k * rules[2].1 + l * rules[3].1;
                let flavor = i * rules[0].2 + j * rules[1].2 + k * rules[2].2 + l * rules[3].2;
                let texture = i * rules[0].3 + j * rules[1].3 + k * rules[2].3 + l * rules[3].3;
                if [capacity, durability, flavor, texture]
                    .into_iter()
                    .any(i32::is_negative)
                {
                    continue;
                } else {
                    max = max.max(capacity * durability * flavor * texture);
                }
            }
        }
    }
    max
}
