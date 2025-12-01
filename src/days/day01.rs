fn parse_turn(line: &str) -> i32 {
    let Ok(turn) = line[1..].parse::<i32>() else {
        unreachable!("Bad input")
    };
    turn
}

pub fn solution1(input: Vec<String>) -> u32 {
    let mut dial = 50i32;
    let mut password = 0;

    for line in input {
        let turn = parse_turn(&line);
        if &line[0..1] == "L" {
            dial -= turn;
        } else {
            dial += turn;
        }

        if dial % 100 == 0 {
            password += 1;
        }
    }
    password
}

pub fn solution2(input: Vec<String>) -> u32 {
    let mut dial = 50i32;
    let mut password = 0;

    for line in input {
        let turn = parse_turn(&line);
        if &line[0..1] == "L" {
            if dial > 0 {
                dial = dial.rem_euclid(100) - 100;
            }
            dial -= turn;
            password -= dial / 100;
        } else {
            if dial <= 0 {
                dial = dial.rem_euclid(100);
            }
            dial += turn;
            password += dial / 100;
        }
        dial %= 100;
    }
    password as u32
}
