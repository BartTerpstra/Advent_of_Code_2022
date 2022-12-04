use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../input/4.txt");

pub type Input = ArrayVec<&'static str, 1000>; //todo example, do change

pub fn read() -> Input {
    INPUT.split('\n').collect()
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

pub fn part1(input: &Input) -> Output {
    Output::U32(-1)
}

pub fn part2(input: &Input) -> Output {
    Output::U32(-1)
}
