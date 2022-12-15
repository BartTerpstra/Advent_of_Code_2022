extern crate core;

mod alt_day_02;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod helper;

use helper::*;

fn main() {
    //todo cli menu
    //todo timing
    //todo (nice to have) automated puzzle downloading and answer checking

    println!("Welcome to Advent of Code 2022 solver");

    let mut func: fn(Part) -> Output;
    let items: Vec<String> = (1..25).map(|x: i32| x.to_string()).collect::<Vec<String>>();

    let input = 1;

    let func = match input {
        _ => day_12::run,
    };

    let answer_one = func(Part::One);
    let answer_two = func(Part::Two);

    println!("************************************************************");
    println!("* Advent of Code: 2022");
    println!("*   Solution for...");
    println!("*     Part One: {}", answer_one);
    println!("*     Part Two: {}", answer_two);
    println!("************************************************************");
}
