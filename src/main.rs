extern crate core;

mod day_1;
mod day_2;
mod day_3;
mod helper;

use helper::*;

fn main() {
    //todo string slice and type conversion for arrayvec
    //todo (nice to have) automated puzzle downloading and answer checking

    println!("Welcome to Advent of Code 2022 solver");

    let mut func: fn(Part) -> Output;
    let items: Vec<String> = (1..25).map(|x: i32| x.to_string()).collect::<Vec<String>>();

    //
    //     let func = match input {
    //         _ => day_1::run,
    //     };
    //
    //     let answer_one = func(Part::One);
    //     let answer_two = func(Part::Two);
    //
    //     println!("************************************************************");
    //     println!("* Advent of Code: 2022");
    //     println!("*   Solution for...");
    //     println!("*     Part One: {}", answer_one);
    //     println!("*     Part Two: {}", answer_two);
    //     println!("************************************************************");
}
