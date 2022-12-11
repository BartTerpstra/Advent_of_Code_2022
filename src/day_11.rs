use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../input/11_test.txt");

pub type Input = Vec<str>;

//todo struct monkey
//has: Vec<Items>, some kind of function, divistibility, left partner, right partner

pub fn read() -> Input {
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
