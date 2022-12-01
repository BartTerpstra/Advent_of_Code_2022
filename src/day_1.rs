use crate::{Output, Part};
use arrayvec::ArrayVec;
use std::io::empty;

const INPUT: &str = include_str!("../input/1.txt");

pub type Input = ArrayVec<&'static str, 2254>;

pub fn read() -> Input {
    //TODO basically just string slice INPUT by line and select and convert to correct type.
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
    let mut sum: u32 = 0;
    let mut result = ArrayVec::<u32, 1000>::new();
    for x in input {
        if x.is_empty() {
            println!("new line");
            result.push(sum); //todo integer.parse(x)
            sum = 0;
        } else {
            println!("value");
            sum += x.parse::<u32>().unwrap(); //todo interger.parse(x);
        }
    }

    Output::U32(*result.iter().max().unwrap())
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}
