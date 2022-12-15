use crate::{Output, Part};
use itertools::Itertools;
use std::usize;

const INPUT: &str = include_str!("../../input/11.txt");

type T = u64;
pub type Input = Vec<Monkey<T>>;

pub struct Monkey<T> {
    items: Vec<T>,
    empathy: Box<dyn Fn(T) -> T>,
    divisibility: T,
    left_partner: usize,
    right_partner: usize,
    activity: u32,
}

pub fn read() -> Input {
    let mut monkeys = Vec::new();
    INPUT.split("\n\n").for_each(|x| {
        let lines: Vec<&str> = x.lines().collect();
        let items = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let components: Vec<&str> = lines[2]
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split(" ")
            .collect();
        assert_eq!(components.len(), 2, "operation string parse error");
        let operator = components[0].chars().next().unwrap();

        let empathy = if components[1] == "old" {
            Box::new(move |x| x * x) as Box<dyn Fn(T) -> T>
        } else {
            let value = components[1].parse::<T>().unwrap();
            match operator {
                '+' => Box::new(move |x| value + x) as Box<dyn Fn(T) -> T>,
                '*' => Box::new(move |x| value * x) as Box<dyn Fn(T) -> T>,
                _ => panic!("expression parse failed"),
            }
        };

        let divisibility = lines[3]
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

        let new_monkey = Monkey::<T> {
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
    let input = &mut read();

    for round in 0..20 {
        for index in 0..input.len() {
            while !input[index].items.is_empty() {
                //take first
                let mut new = 0;
                {
                    let monkey = &mut input[index];
                    let item = monkey.items[0];
                    monkey.items.remove(0);
                    monkey.activity += 1;

                    let func = &monkey.empathy;
                    new = func(item) / 3;
                }
                {
                    let move_to = if new % input[index].divisibility == 0 {
                        input[index].left_partner
                    } else {
                        input[index].right_partner
                    };
                    input[move_to].items.push(new);
                }
            }
        }
    }

    let answer = input
        .iter()
        .map(|x| x.activity)
        .sorted()
        .rev()
        .take(2)
        .product();

    Output::U32(answer)
}

pub fn part2() -> Output {
    let input = &mut read();

    let BIG_BAD_DIV = input.iter().map(|x| x.divisibility).product::<T>();
    println!("BIG BAD {}", BIG_BAD_DIV);

    for round in 0..10000 {
        for index in 0..input.len() {
            while !input[index].items.is_empty() {
                //take first
                let mut new = 0;
                {
                    let monkey = &mut input[index];
                    let item = monkey.items[0];
                    monkey.items.remove(0);
                    monkey.activity += 1;

                    let func = &monkey.empathy;
                    new = func(item) % BIG_BAD_DIV;
                }
                {
                    //assert 0 % y == 0 == true
                    let move_to = if new % input[index].divisibility == 0 {
                        input[index].left_partner
                    } else {
                        input[index].right_partner
                    };
                    input[move_to].items.push(new);
                }
            }
        }
    }

    let answer = input
        .iter()
        .map(|x| x.activity as T)
        .sorted()
        .rev()
        .take(2)
        .product();

    Output::U64(answer)
}
