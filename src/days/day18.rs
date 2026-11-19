use itertools::Itertools;

fn parse(input: &[String]) -> Vec<Vec<bool>> {
    input
        .iter()
        .map(|l| l.chars().map(|c| c == '#').collect_vec())
        .collect_vec()
}

fn step(lights: &mut [Vec<bool>], stuck: bool) {
    let mut neighbors = vec![vec![0u8; 100]; 100];
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    (0..100).cartesian_product(0..100).for_each(|(i, j)| {
        let n = [-1, 0, 1]
            .into_iter()
            .cartesian_product([-1, 0, 1])
            .filter(|(a, b)| {
                !(*a == 0 && *b == 0)
                    && (0..100).contains(&(i + a))
                    && (0..100).contains(&(j + b))
                    && lights[(i + a) as usize][(j + b) as usize]
            })
            .count();
        neighbors[i as usize][j as usize] += n as u8;
    });
    (0..100).cartesian_product(0..100).for_each(|(i, j)| {
        if lights[i][j] {
            lights[i][j] = [2, 3].contains(&neighbors[i][j]);
        } else {
            lights[i][j] = neighbors[i][j] == 3;
        }
        if stuck && [0, 99].contains(&i) && [0, 99].contains(&j) {
            lights[i][j] = true;
        }
    });
}

pub fn part1(input: &[String]) -> usize {
    let mut lights = parse(input);
    for _ in 0..100 {
        step(&mut lights, false);
    }
    lights.into_iter().flatten().filter(|&b| b).count()
}

pub fn part2(input: &[String]) -> usize {
    let mut lights = parse(input);
    [0, 99]
        .into_iter()
        .cartesian_product([0, 99])
        .for_each(|(i, j)| lights[i][j] = true);
    for _ in 0..100 {
        step(&mut lights, true);
    }
    lights.into_iter().flatten().filter(|&b| b).count()
}

// fn print(light: &Vec<Vec<bool>>) {
//     light
//         .iter()
//         .map(|l| {
//             l.iter()
//                 .map(|&l| if l { '#' } else { '.' })
//                 .collect::<String>()
//         })
//         .for_each(|l| println!("{}", l));
//     println!()
// }
