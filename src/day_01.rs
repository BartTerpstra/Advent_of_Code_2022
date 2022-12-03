use crate::{Output, Part};
use arrayvec::ArrayVec;
use itertools::Itertools;

const INPUT: &str = include_str!("../input/1.txt");

pub type Input = ArrayVec<&'static str, 2254>;

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
    //from calorie streaks seperated by empty lines into a list of calorie totals.
    let mut sum: u32 = 0;
    let mut elf_totals = ArrayVec::<u32, 1000>::new();
    for x in input {
        if x.is_empty() {
            elf_totals.push(sum);
            sum = 0;
        } else {
            sum += x.parse::<u32>().unwrap();
        }
    }

    Output::U32(*elf_totals.iter().max().unwrap())
}

pub fn part2(input: &Input) -> Output {
    //from calorie streaks seperated by empty lines into a list of calorie totals.
    let mut sum: u32 = 0;
    let mut elf_totals = ArrayVec::<u32, 1000>::new();
    for x in input {
        if x.is_empty() {
            elf_totals.push(sum);
            sum = 0;
        } else {
            sum += x.parse::<u32>().unwrap();
        }
    }

    //sum the 3 highest
    let mut output = 0;
    elf_totals.sort();
    for _ in 1..=3 {
        let max_index = elf_totals.iter().position_max().unwrap();
        output += elf_totals.pop_at(max_index).unwrap();
        println!("{}", output);
    }
    Output::U32(output)
}
