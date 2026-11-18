use md5::{Digest, Md5};

pub fn part1(input: &[String]) -> usize {
    let line = input.first().unwrap();
    for n in 1.. {
        let md5 = Md5::digest(format!("{line}{n}"));
        if md5.starts_with(&[0, 0]) && md5[2] & 0xF0 == 0 {
            return n;
        }
    }
    unreachable!()
}

pub fn part2(input: &[String]) -> usize {
    let line = input.first().unwrap();
    for n in 1.. {
        let md5 = Md5::digest(format!("{line}{n}"));
        if md5.starts_with(&[0, 0, 0]) {
            return n;
        }
    }
    unreachable!()
}
