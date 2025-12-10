use itertools::Itertools;

pub fn part1(input: Vec<String>) -> usize {
    let pairs: Vec<(usize, usize)> = input
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
        let x1 = p1.0.min(p2.0);
        let x2 = p1.0.max(p2.0);
        let y1 = p1.1.min(p2.1);
        let y2 = p1.1.max(p2.1);
        max = max.max((x2 - x1 + 1) * (y2 - y1 + 1));
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

    let mut max = 0;
    for pair in red.iter().combinations(2) {
        let [p1, p2] = pair[0..2] else { unreachable!() };
        let x1 = p1.0.min(p2.0);
        let x2 = p1.0.max(p2.0);
        let y1 = p1.1.min(p2.1);
        let y2 = p1.1.max(p2.1);
        // Logic inspired by the Sutherland–Hodgman algorithm
        if red.iter().enumerate().all(|(k, &(x, y))| {
            match (((x1 + 1)..x2).contains(&x), ((y1 + 1)..y2).contains(&y)) {
                (true, true) => false, // Point inside
                (true, false) => {
                    let (nx, ny) = red[(k + 1) % red.len()];
                    if x == nx {
                        let range = y.min(ny)..=y.max(ny);
                        // Check if line crosses horizontally
                        return !(y1 != y2 && range.contains(&y1) && range.contains(&y2));
                    }
                    true
                }
                (false, true) => {
                    let (nx, ny) = red[(k + 1) % red.len()];
                    if y == ny {
                        let range = x.min(nx)..=x.max(nx);
                        // Check if line crosses vertically
                        return !(x1 != x2 && range.contains(&x1) && range.contains(&x2));
                    }
                    true
                }
                (false, false) => true, // Completely outside
            }
        }) {
            max = max.max((y2 - y1 + 1) * (x2 - x1 + 1));
        }
    }
    max
}
