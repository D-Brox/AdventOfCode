use itertools::Itertools;

pub fn part1(input: Vec<String>) -> u32 {
    let mut accessible = 0;
    let table = input
        .iter()
        .map(|l| l.chars().map(|c| (c == '@') as u8).collect_vec())
        .collect_vec();

    let h = table.len();
    let w = table[0].len();
    for (i, j) in [0, h - 1].into_iter().cartesian_product([0, w - 1]) {
        if table[i][j] == 1 {
            accessible += 1;
        }
    }

    for j in 1..(w - 1) {
        if table[0][j] == 1 {
            let neighbors =
                table[0][j - 1] + table[0][j + 1] + table[1][j - 1] + table[1][j] + table[1][j + 1];
            if neighbors < 4 {
                accessible += 1;
            }
        }
        if table[h - 1][j] == 1 {
            let neighbors = table[h - 2][j - 1]
                + table[h - 2][j]
                + table[h - 2][j + 1]
                + table[h - 1][j - 1]
                + table[h - 1][j + 1];
            if neighbors < 4 {
                accessible += 1;
            }
        }
    }
    for i in 1..(h - 1) {
        if table[i][0] == 1 {
            let neighbors =
                table[i - 1][0] + table[i - 1][1] + table[i][1] + table[i + 1][0] + table[i + 1][1];
            if neighbors < 4 {
                accessible += 1;
            }
        }
        if table[i][w - 1] == 1 {
            let neighbors = table[i - 1][w - 2]
                + table[i - 1][w - 1]
                + table[i][w - 2]
                + table[i + 1][w - 2]
                + table[i + 1][w - 1];
            if neighbors < 4 {
                accessible += 1;
            }
        }
    }

    for (i, j) in (1..(h - 1)).cartesian_product(1..(w - 1)) {
        if table[i][j] == 1 {
            let neighbors = table[i - 1][j - 1]
                + table[i - 1][j]
                + table[i - 1][j + 1]
                + table[i][j - 1]
                + table[i][j + 1]
                + table[i + 1][j - 1]
                + table[i + 1][j]
                + table[i + 1][j + 1];
            if neighbors < 4 {
                accessible += 1;
            }
        }
    }
    accessible
}

pub fn part2(input: Vec<String>) -> u32 {
    let mut accessible = 0;
    let mut table = input
        .iter()
        .map(|l| l.chars().map(|c| (c == '@') as u8).collect_vec())
        .collect_vec();
    let h = table.len();
    let w = table[0].len();
    for (i, j) in [0, h - 1].into_iter().cartesian_product([0, w - 1]) {
        if table[i][j] == 1 {
            table[i][j] = 0;
            accessible += 1;
        }
    }
    let mut previous = 0;
    let mut top = (1..(w - 1)).filter(|&j| table[0][j] == 1).collect_vec();
    let mut bottom = (1..(w - 1)).filter(|&j| table[h - 1][j] == 1).collect_vec();
    let mut left = (1..(h - 1)).filter(|&i| table[i][0] == 1).collect_vec();
    let mut right = (1..(h - 1)).filter(|&i| table[i][w - 1] == 1).collect_vec();
    let mut center = (1..(h - 1))
        .cartesian_product(1..(w - 1))
        .filter(|&(i, j)| table[i][j] == 1)
        .collect_vec();
    while previous != accessible {
        previous = accessible;
        for &j in &top {
            let neighbors =
                table[0][j - 1] + table[0][j + 1] + table[1][j - 1] + table[1][j] + table[1][j + 1];
            if neighbors < 4 {
                table[0][j] = 0;
                accessible += 1;
            }
        }
        for &j in &bottom {
            let neighbors = table[h - 2][j - 1]
                + table[h - 2][j]
                + table[h - 2][j + 1]
                + table[h - 1][j - 1]
                + table[h - 1][j + 1];
            if neighbors < 4 {
                table[h - 1][j] = 0;
                accessible += 1;
            }
        }
        for &i in &left {
            let neighbors =
                table[i - 1][0] + table[i - 1][1] + table[i][1] + table[i + 1][0] + table[i + 1][1];
            if neighbors < 4 {
                table[i][0] = 0;
                accessible += 1;
            }
        }
        for &i in &right {
            let neighbors = table[i - 1][w - 2]
                + table[i - 1][w - 1]
                + table[i][w - 2]
                + table[i + 1][w - 2]
                + table[i + 1][w - 1];
            if neighbors < 4 {
                table[i][w - 1] = 0;
                accessible += 1;
            }
        }
        for &(i, j) in &center {
            let neighbors = table[i - 1][j - 1]
                + table[i - 1][j]
                + table[i - 1][j + 1]
                + table[i][j - 1]
                + table[i][j + 1]
                + table[i + 1][j - 1]
                + table[i + 1][j]
                + table[i + 1][j + 1];
            if neighbors < 4 {
                table[i][j] = 0;
                accessible += 1;
            }
        }
        top.retain(|&j| table[0][j] == 1);
        bottom.retain(|&j| table[h - 1][j] == 1);
        left.retain(|&i| table[i][0] == 1);
        right.retain(|&i| table[i][w - 1] == 1);
        center.retain(|&(i, j)| table[i][j] == 1);
    }

    accessible
}
