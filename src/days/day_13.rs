use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../../input/13_test.txt");

pub type Input = Vec<(String, String)>;

pub fn read() -> Input {
    let answer: Vec<(String, String)> = Vec::new();
    INPUT
        .split("\n\n")
        .map(|x| {
            let mut first = true;
            let mut answer: (String, String) = ("".to_string(), "".to_string());
            let split = x.lines().take(2).map(|x| {
                if first {
                    first = false;
                    answer.0 = x.to_string();
                } else {
                    answer.1 = x.to_string();
                }
            });
            answer
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
    Output::U32(1)
}

pub fn part2(input: &Input) -> Output {
    Output::U32(1)
}
