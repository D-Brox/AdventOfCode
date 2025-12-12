pub fn part1(input: Vec<String>) -> usize {
    let grids = input.rsplitn(2, String::is_empty).next().unwrap();
    let mut count = 0;
    for grid in grids {
        let (s, t) = grid.split_once(':').unwrap();
        let size = s
            .split_once('x')
            .map(|(a, b)| a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap())
            .unwrap();
        if t.split(' ')
            .filter_map(|n| n.parse::<usize>().ok())
            .map(|n| 7 * n)
            .sum::<usize>()
            <= size
        {
            count += 1;
        }
    }
    count
}

pub fn part2(_input: Vec<String>) {}
