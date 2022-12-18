use crate::{Output, Part};
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::ops::Index;

const INPUT: &str = include_str!("../../input/13_test.txt");

pub type Input = ArrayVec<&'static str, 450>;

pub fn read() -> Input {
    INPUT.lines().collect()
}
/** https://adventofcode.com/2022/day/13
*/
pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

pub fn part1(input: &Input) -> Output {
    let mut answers: Vec<usize> = Vec::new();

    let pairs: Vec<&[&str]> = input.chunks(2).filter(|x| x.is_empty()).take(2).collect();
    //assert Vec<&[&str;2]>

    for pair_index in 0..pairs.len() {
        let left: Vec<char>= pairs[pair_index][0].chars().collect();
        let right: Vec<char> = pairs[pair_index][1].chars().collect();

        //packet order comparison. left number should be smaller than right to be ordered
        //if right length runs out before left length, right is smaller, therefore unordered
        let mut found_unordered = false;
        for str_index in 0..left.len() {
            //check if right has an item.
            if str_index >= right.len() {
                found_unordered = true;
                break;
            }

            /**
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]
*/
            //find a number for left and right
            let left = left[0];
            let right = right[0];

            match left{
                '[' =>,
                ']' =>,
                _ =>
            }
        }

        if !found_unordered {
            //found ordered
            answers.push(pair_index);
        }
    }

    //vec to string
    let mut answer = "".to_string();
    Output::String(answer)
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}
