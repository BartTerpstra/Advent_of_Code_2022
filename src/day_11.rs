use crate::{Output, Part};
use arrayvec::ArrayVec;
use itertools::{Itertools, PeekingNext};
use std::borrow::{Borrow, BorrowMut};
use std::env::Args;
use std::ops::{Add, Deref, Mul};
use std::usize;

const INPUT: &str = include_str!("../input/11_test.txt");

pub type Input = Vec<Monkey>;

struct Monkey {
    items: Vec<u32>,
    empathy: Box<dyn Fn(u32) -> u32>,
    divisibility: u32,
    left_partner: usize,
    right_partner: usize,
    activity: u32,
}

pub fn read() -> Input {
    let mut monkeys = Vec::new();
    INPUT.split("\n\n").for_each(|x| {
        let lines: Vec<&str> = x.lines().collect();
        let items: Vec<u32> = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let components: Vec<&str> = lines[2]
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split(" ")
            .collect();
        assert_eq!(components.len(), 2, "operation string parse error");
        let operator = components[0].chars().next().unwrap();

        let mut empathy: Box<dyn Fn(u32) -> u32>;
        if components[1] == "old" {
            empathy = Box::new(move |x: u32| x * x)
        } else {
            let value = components[1].parse::<u32>().unwrap();
            empathy = match operator {
                '+' => Box::new(move |x: u32| value + x),
                '*' => Box::new(move |x: u32| value * x),
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
            activity: 0,
        };
        monkeys.push(new_monkey);
    });

    monkeys
}

pub fn run(part: Part) -> Output {
    match part {
        Part::One => part1(),
        Part::Two => part2(),
    }
}

pub fn part1() -> Output {
    let mut input = read();

    for round in 0..20 {
        for monkey in &input {
            while !monkey.items.is_empty() {
                //take first

                let item = *monkey.items.get(0).unwrap();
                monkey.items.remove(0);
                monkey.activity += 1;

                let func = &monkey.empathy;
                let new: u32 = func(item);

                {
                    if new % monkey.divisibility == 0 {
                        input[monkey.left_partner].items.push(new);
                    } else {
                        input[monkey.right_partner].items.push(new);
                    }
                }
            }
        }
    }
    Output::U32(0)
}

pub fn part2() -> Output {
    Output::U32(0)
}
