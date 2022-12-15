use crate::{Output, Part};
use arrayvec::ArrayVec;
use std::collections::HashSet;
use std::iter::Iterator;

const INPUT: &str = include_str!("../input/3.txt");

pub type Input = ArrayVec<&'static str, 301>; //todo example, do change

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

//too low 8383
pub fn part1(input: &Input) -> Output {
    Output::U32(
        input
            .iter()
            .map(|x| char_in_both_halves(*x))
            .map(|x| to_digit(&x))
            .sum::<u32>(),
    )
}

pub fn part2(input: &Input) -> Output {
    let mut sum = 0;
    for x in 0..input.len() / 3 {
        sum += to_digit(&char_in_all_strings(
            input[x * 3],
            input[x * 3 + 1],
            input[x * 3 + 2],
        ));
    }
    return Output::U32(sum);
}

fn char_in_all_strings(first: &str, second: &str, third: &str) -> char {
    for x in first.chars().collect::<HashSet<char>>() {
        for y in second.chars().collect::<HashSet<char>>() {
            for z in third.chars().collect::<HashSet<char>>() {
                if x == y && y == z && x == z {
                    return x;
                }
            }
        }
    }
    panic!("you messed up day 3 part 2")
}

fn char_in_both_halves(rucksack: &str) -> char {
    //fold the string back onto itself
    //first check last and first with eachother
    //then check first with last-1 and first+1 with last
    //etc
    //return first match value found

    let charsack: Vec<char> = rucksack.chars().collect();
    for y in (charsack.len() / 2)..charsack.len() {
        for g in 0..charsack.len() / 2 {
            if charsack[y] == charsack[g] {
                return charsack[g];
            }
        }
    }

    println!("failed input: {}", rucksack);
    println!("length: {}", rucksack.len());

    panic!("search failed unexpectedly");
}
fn to_digit(character: &char) -> u32 {
    let chars_to_num: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let num_to_chars: Vec<u32> = (1..=52).collect();

    let mut index = 0;
    for x in chars_to_num {
        if x == *character {
            break;
        } else {
            index += 1;
        }
    }

    num_to_chars[index]
}
