pub fn part1(input: &[String]) -> usize {
    let code: usize = input.iter().map(String::len).sum();
    let esc: usize = input
        .iter()
        .map(|l| {
            l.chars()
                .fold((0, String::new()), |(acc, prev), c| match c {
                    '\\' if prev == "\\" => (acc + 1, String::new()),
                    '\\' => (acc, "\\".into()),
                    'x' if prev == "\\" => (acc, "\\x".into()),
                    'a'..='z' | '0'..='9' if prev.starts_with("\\x") => {
                        if prev.len() == 3 {
                            (acc + 1, String::new())
                        } else {
                            (acc, format!("{}{c}", prev.clone()))
                        }
                    }
                    _ => (acc + 1, String::new()),
                })
                .0
                - 2
        })
        .sum();
    code - esc
}

pub fn part2(input: &[String]) -> usize {
    let code: usize = input.iter().map(String::len).sum();
    let esc: usize = input
        .iter()
        .map(|l| {
            l.chars().fold(0, |acc, c| match c {
                '\\' | '"' => acc + 2,
                _ => acc + 1,
            }) + 2
        })
        .sum();
    esc - code
}
