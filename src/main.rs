extern crate core;

mod day_01;
mod day_02;
mod day_03;
mod helper;

use helper::*;

fn main() {
    //todo cli menu
    //todo (nice to have) automated puzzle downloading and answer checking

    println!("Welcome to Advent of Code 2022 solver");

    let mut func: fn(Part) -> Output;
    let items: Vec<String> = (1..25).map(|x: i32| x.to_string()).collect::<Vec<String>>();

    let input = 1;

    let func = match input {
        _ => day_01::run,
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
