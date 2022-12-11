use crate::{Output, Part};
use arrayvec::ArrayVec;
use itertools::{Itertools, PeekingNext};
use std::ops::{Add, Mul};
use std::usize;

const INPUT: &str = include_str!("../input/11_test.txt");

pub type Input = Vec<Monkey>;

struct Monkey {
    items: Vec<u8>,
    empathy: Box<dyn Fn(usize) -> usize>,
    divisibility: u32,
    left_partner: usize,
    right_partner: usize,
}

pub fn read() -> Input {
    let mut monkeys = Vec::new();
    INPUT.split("\n\n").for_each(|x| {
        let lines: Vec<&str> = x.lines().collect();
        let items: Vec<u8> = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        let components: Vec<&str> = lines[2]
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split(" ")
            .collect();
        assert_eq!(components.len(), 2, "operation string parse error");
        let operator = components[0].chars().next().unwrap();

        let mut empathy: Box<dyn Fn(usize) -> usize>;
        if components[1] == "old" {
            empathy = Box::new(move |x: usize| x * x)
        } else {
            let value = components[1].parse::<usize>().unwrap();
            empathy = match operator {
                '+' => Box::new(move |x: usize| value + x),
                '*' => Box::new(move |x: usize| value * x),
                _ => panic!("expression parse failed"),
            };
        }

        let divisibility: u32 = lines[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let left_partner: usize = lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let right_partner: usize = lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let new_monkey = Monkey {
            items,
            empathy,
            divisibility,
            left_partner,
            right_partner,
        };
        monkeys.push(new_monkey);
    });

    monkeys
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
