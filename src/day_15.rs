use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../input/15_test.txt");

pub type Input = ArrayVec<&'static str, 1024>; //todo example, do change

pub fn read() -> Input {
    //TODO basically just string slice INPUT by line and select and convert to correct type.
    INPUT.lines().collect()
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
