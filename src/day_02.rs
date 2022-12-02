use crate::day_02::Hand::{Paper, Rock, Scissors};
use crate::day_02::MatchResult::{Draw, Loss, Win};
use crate::{Output, Part};
use arrayvec::ArrayVec;
use std::borrow::Borrow;
use std::ops::Deref;

const INPUT: &str = include_str!("../input/2.txt");

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

//too high 10913
//too high 10532
//too high 15488

//too high 13709
//too high 14090
//too low 9134
pub fn part1(input: &Input) -> Output {
    Output::U32(
        input
            .iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .map(|f| (map_hand(f[2], 0), map_hand(f[0], 0)))
            .map(|pair| {
                get_result_value(who_wins(pair.0.borrow(), pair.1.borrow()))
                    + get_value(pair.0.borrow())
            })
            .sum(),
    )
}

pub fn part2(input: &Input) -> Output {
    // Output::U32(input.iter().map(|pair| {})).sum();
    return Output::U32(0);
}

fn map_hand(signifier: char, offset: usize) -> Hand {
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

fn get_match_result(entry: char) -> MatchResult {
    Win
}

fn get_result_value(result: MatchResult) -> u32 {
    match result {
        Win => 6,
        Draw => 3,
        Loss => 0,
    }
}
fn get_value(thrown: &Hand) -> u32 {
    match thrown {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}
