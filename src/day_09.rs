use crate::day_09::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../input/9.txt");

pub type Input = ArrayVec<Operation, 2000>;

struct Operation {
    direction: Direction,
    amount: u8,
}

enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

fn to_direction(char: char) -> Direction {
    match char {
        'L' => LEFT,
        'R' => RIGHT,
        'U' => UP,
        'D' => DOWN,
        _ => panic!("input processing error. no such direction"),
    }
}
pub fn read() -> Input {
    INPUT
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.split(' '))
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
