use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../../input/2.txt");

pub type Input = ArrayVec<&'static str, 2500>; //todo example, do change

pub fn read() -> Input {
    //TODO basically just string slice INPUT by line and select and convert to correct type.
    INPUT.lines().collect()
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

pub fn part1(input: &Input) -> Output {
    return Output::U32(input.iter().fold(0, |acc, str| match *str {
        "A X" => acc + 3 + 1, //rock rock (draw)
        "A Y" => acc + 6 + 2, //rock paper (wom)
        "A Z" => acc + 0 + 3, //rock scissors (loss)
        "B X" => acc + 0 + 1, //paper rock (loss)
        "B Y" => acc + 3 + 2, //paper paper  (draw)
        "B Z" => acc + 6 + 3, //paper scissors (win)
        "C X" => acc + 6 + 1, //scissors rock (win)
        "C Y" => acc + 0 + 2, //scissors paper (loss)
        "C Z" => acc + 3 + 3, //scissors scissors (draw)
        _ => acc,
    }));
}

pub fn part2(input: &Input) -> Output {
    return Output::U32(input.iter().fold(0, |acc, str| match *str {
        "A X" => acc + 0 + 3, //rock scissors (loss)
        "A Y" => acc + 3 + 1, //rock rock (draw)
        "A Z" => acc + 6 + 2, //rock paper (win)
        "B X" => acc + 0 + 1, //paper rock (loss)
        "B Y" => acc + 3 + 2, //paper paper  (draw)
        "B Z" => acc + 6 + 3, //paper scissors (win)
        "C X" => acc + 0 + 2, //scissors paper (loss)
        "C Y" => acc + 3 + 3, //scissors scissors (draw)
        "C Z" => acc + 6 + 1, //scissors rock (win)
        _ => acc,
    }));
}
