use crate::days::day_02::Hand::{Paper, Rock, Scissors};
use crate::days::day_02::MatchResult::{Draw, Loss, Win};
use crate::{Output, Part};
use arrayvec::ArrayVec;
use std::borrow::Borrow;

const INPUT: &str = include_str!("../../input/2.txt");

pub type Input = ArrayVec<&'static str, 2500>; //todo example, do change

pub fn read() -> Input {
    //by line, split line, to chararray, to hand tuple
    INPUT.lines().collect()
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}

pub fn part1(input: &Input) -> Output {
    Output::U32(
        input
            .iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .map(|f| (map_hand(&f[2], 0), map_hand(&f[0], 0)))
            .map(|pair| {
                get_result_value(&who_wins(pair.0.borrow(), pair.1.borrow()))
                    + get_value(pair.0.borrow())
            })
            .sum(),
    )
}

//NOTE this solution gives the wrong answer. the right answer is 10560.
//maybe come back later and look up why
pub fn part2(input: &Input) -> Output {
    Output::U32(
        input
            .iter()
            .map(|x| x.chars().collect())
            .map(|x: Vec<char>| {
                get_result_value(&get_match_result(&x[2]))
                    + get_complement_value(&map_hand(&x[0], 0), &get_match_result(&x[2]))
            })
            .sum(),
    )
}

fn map_hand(signifier: &char, offset: usize) -> Hand {
    let mut some_answers = vec![Rock, Paper, Scissors];
    some_answers.rotate_left(offset);

    match signifier {
        'X' => some_answers.remove(0),
        'Y' => some_answers.remove(1),
        'Z' => some_answers.remove(2),
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _ => panic!("invalid input"),
    }
}

fn who_wins(you: &Hand, opponent: &Hand) -> MatchResult {
    match you {
        Rock => match opponent {
            Paper => Loss,
            Scissors => Win,
            Rock => Draw,
        },
        Paper => match opponent {
            Scissors => Loss,
            Rock => Win,
            Paper => Draw,
        },
        Scissors => match opponent {
            Rock => Loss,
            Paper => Win,
            Scissors => Draw,
        },
    }
}

fn get_match_result(entry: &char) -> MatchResult {
    match entry {
        'X' => Loss,
        'Y' => Draw,
        'Z' => Win,
        _ => {
            println!("{}", entry);
            panic!("invalid data")
        }
    }
}

fn get_result_value(result: &MatchResult) -> u32 {
    match result {
        Win => 6,
        Draw => 3,
        Loss => 0,
    }
}

//given another players hand and the result of the match from your perspective, get the value of the hand you threw
fn get_complement_value(other: &Hand, result: &MatchResult) -> u32 {
    match result {
        Win => (get_value(other) + 1) % 3,
        Draw => get_value(other),
        Loss => (get_value(other) + 2) % 3,
    }
}

fn get_value(thrown: &Hand) -> u32 {
    match thrown {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}
