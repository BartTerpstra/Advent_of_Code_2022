use crate::Grid::Grid;
use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../../input/15_test.txt");

pub type Input = Grid<bool>;

pub fn read() -> Input {
    Grid::new_w_offset(0, 0, 20, 20)
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
