fn sum_factors(mut n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let mut factors = vec![];
    let mut d: u32 = 2;
    while d.pow(2) < n {
        let mut exp = 0u32;
        while n.is_multiple_of(d) {
            n /= d;
            exp += 1;
        }
        if exp != 0 {
            factors.push((d, exp));
        }
        d += 1;
    }
    if n > 1 {
        factors.push((n, 1));
    }

    let mut n = 1;
    for (p, exp) in factors {
        n *= (p.pow(exp + 1) - 1) / (p - 1);
    }
    n
}

fn sum_factors_till_50(n: u32) -> u32 {
    let mut sum = 0;
    let mut d: u32 = 1;
    while d.pow(2) <= n {
        if n.is_multiple_of(d) {
            let q = n / d;
            if q <= 50 {
                sum += d;
            }
            if d != q && d <= 50 {
                sum += q;
            }
        }
        d += 1;
    }

    sum
}

pub fn part1(input: &[String]) -> u32 {
    let input = input.first().unwrap().parse::<u32>().unwrap();
    (1..=input / 10)
        .find(|i| sum_factors(*i) * 10 > input)
        .unwrap()
}

pub fn part2(input: &[String]) -> u32 {
    let input = input.first().unwrap().parse::<u32>().unwrap();
    (1..=input / 11)
        .find(|i| sum_factors_till_50(*i) * 11 >= input)
        .unwrap()
}
