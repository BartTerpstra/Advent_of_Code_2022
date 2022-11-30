use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../input/1.txt");

pub type Input = ArrayVec<u8, 1024>; //example, do change

pub fn read() -> Input {
    INPUT
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

pub fn part1(input: Input) -> Output {
    unimplemented!()
}

pub fn part2(input: Input) -> Output {
    unimplemented!()
}
