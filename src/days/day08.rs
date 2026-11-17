pub fn part1(input: Vec<String>) -> usize {
    let code: usize = input.iter().map(|l| l.len()).sum();
    let esc: usize = input
        .iter()
        .map(|l| {
            l.chars()
                .fold((0, "".to_string()), |(acc, prev), c| match c {
                    '\\' if prev == "\\" => (acc + 1, "".into()),
                    '\\' => (acc, "\\".into()),
                    'x' if prev == "\\" => (acc, "\\x".into()),
                    'a'..='z' | '0'..='9' if prev.starts_with("\\x") => {
                        if prev.len() == 3 {
                            (acc + 1, "".into())
                        } else {
                            (acc, format!("{}{c}", prev.clone()))
                        }
                    }
                    _ => (acc + 1, "".into()),
                })
                .0
                - 2
        })
        .sum();
    code - esc
}

pub fn part2(input: Vec<String>) -> usize {
    let code: usize = input.iter().map(|l| l.len()).sum();
    let esc: usize = input
        .iter()
        .map(|l| {
            l.chars().fold(0, |acc, c| match c {
                '\\' => acc + 2,
                '"' => acc + 2,
                _ => acc + 1,
            }) + 2
        })
        .sum();
    esc - code
}
