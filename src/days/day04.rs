use md5::{Digest, Md5};

pub fn part1(input: Vec<String>) -> usize {
    let line = input.first().unwrap();
    (1..)
        .find(|n| {
            let md5 = Md5::digest(format!("{line}{n}"));
            md5.starts_with(&[0, 0]) && md5[2] & 0xF0 == 0
        })
        .unwrap()
}

pub fn part2(input: Vec<String>) -> usize {
    let line = input.first().unwrap();
    (1..)
        .find(|n| {
            let md5 = Md5::digest(format!("{line}{n}"));
            md5.starts_with(&[0, 0, 0])
        })
        .unwrap()
}
