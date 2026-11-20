use itertools::Itertools;

use crate::days::day23::Instruction::{Half, Increment, JEven, JOne, Jump, Triple};

#[derive(Debug)]
enum Instruction {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(i32),
    JEven(usize, i32),
    JOne(usize, i32),
}

fn parse(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|l| {
            let mut l = l.split(' ');
            let i = l.next().unwrap();
            let r = |c: char| usize::from(c != 'a');
            match i {
                "hlf" => Half(r(l.next().unwrap().chars().next().unwrap())),
                "tpl" => Triple(r(l.next().unwrap().chars().next().unwrap())),
                "inc" => Increment(r(l.next().unwrap().chars().next().unwrap())),
                "jmp" => Jump(l.next().unwrap().parse().unwrap()),
                "jie" => JEven(
                    r(l.next().unwrap().chars().next().unwrap()),
                    l.next().unwrap().parse().unwrap(),
                ),
                "jio" => JOne(
                    r(l.next().unwrap().chars().next().unwrap()),
                    l.next().unwrap().parse().unwrap(),
                ),
                _ => unreachable!(),
            }
        })
        .collect_vec()
}

fn run(instructions: &[Instruction], mut regs: [u32; 2]) -> u32 {
    let mut i = 0;
    while let Some(inst) = instructions.get(i) {
        #[allow(
            clippy::cast_sign_loss,
            clippy::cast_possible_wrap,
            clippy::cast_possible_truncation
        )]
        match inst {
            Half(r) => regs[*r] /= 2,
            Triple(r) => regs[*r] *= 3,
            Increment(r) => regs[*r] += 1,
            Jump(offset) => {
                i = (i as i32 + offset) as usize;
                continue;
            }
            JEven(r, offset) if regs[*r] % 2 == 0 => {
                i = (i as i32 + offset) as usize;
                continue;
            }
            JOne(r, offset) if regs[*r] == 1 => {
                i = (i as i32 + offset) as usize;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    regs[1]
}

pub fn part1(input: &[String]) -> u32 {
    let instructions = parse(input);
    run(&instructions, [0, 0])
}

pub fn part2(input: &[String]) -> u32 {
    let instructions = parse(input);
    run(&instructions, [1, 0])
}
