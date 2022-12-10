use crate::day_10::InstructionType::{ADDX, NOP};
use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../input/10.txt");

pub type Input = Vec<Instruction>; //todo example, do change

struct Instruction {
    instruction_type: InstructionType,
    value: i32,
}

enum InstructionType {
    ADDX,
    NOP,
}
pub fn read() -> Input {
    //TODO basically just string slice INPUT by line and select and convert to correct type.
    INPUT
        .split('\n')
        .map(|x| {
            let arr: Vec<&str> = x.split(' ').take(2).collect();
            let instruction = match arr[0] {
                "addx" => Instruction {
                    instruction_type: ADDX,
                    value: arr[1].parse().unwrap(),
                },
                "noop" => Instruction {
                    instruction_type: NOP,
                    value: 0,
                },
                _ => {
                    panic!("input read error")
                }
            };
            instruction
        })
        .collect()
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

pub fn part1(input: &Input) -> Output {
    Output::U32(0)
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}
