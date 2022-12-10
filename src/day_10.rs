use crate::day_10::InstructionType::{ADDX, NOP};
use crate::{Output, Part};
use arrayvec::ArrayVec;
use derive_more::Display;
use itertools::Itertools;
use regex::internal::Inst;
use std::ops::Add;

const INPUT: &str = include_str!("../input/10.txt");

pub type Input = Vec<Instruction>; //todo example, do change

struct Instruction {
    instruction_type: InstructionType,
    value: i32,
}

#[derive(Display)]
enum InstructionType {
    ADDX,
    NOP,
}
pub fn read() -> Input {
    INPUT
        .split('\n')
        .filter(|x| !x.is_empty())
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

//too high 33600
//incorrect -19220
//incorrect -14800
pub fn part1(input: &Input) -> Output {
    let mut answers: Vec<i32> = Vec::new();
    let mut register = 1;
    let mut pending = -1;

    let pol_cycles: Vec<u8> = vec![20, 60, 100, 140, 180, 220];
    let mut input = input.iter();
    let mut occupied: u32 = 0;

    for x in 1..u8::MAX {
        //evaluate register
        if pol_cycles.contains(&x) {
            println!("register {}", register);
            println!("x {}", x);
            answers.push(x as i32 * register)
        }

        if occupied == 0 {
            let potential_instruction = input.next();
            if potential_instruction.is_none() {
                break;
            }
            let instruction = potential_instruction.unwrap();

            match instruction.instruction_type {
                ADDX => {
                    pending = instruction.value;
                    occupied += 1;
                }
                NOP => {}
            }
        } else {
            occupied -= 1;
            if occupied == 0 {
                register += pending;
            }
        }
    }

    Output::I32(answers.iter().sum())
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}
