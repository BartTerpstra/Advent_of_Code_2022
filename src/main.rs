extern crate core;

mod day_1;
mod day_2;
mod day_3;
mod helper;

use helper::*;
use std::fs;
use std::io::{stdin, Empty};
use std::str;

fn main() {
    //todo file read
    //todo (nice to have) automated puzzle downloading and answer checking

    println!("Welcome to Advent of Code 2022 solver");

    let mut has_selected_day: bool = false;
    let mut func: fn(Part) -> Output;
    while !has_selected_day {
        println!("Which day would you like to run?");

        let mut buffer = String::new();

        // `read_line` returns `Result` of bytes read
        stdin().read_line(&mut buffer)?;

        func = match buffer.trim_end() {
            "1" => day_1::run,
            "2" => day_2::run,
            "3" => day_3::run,
            _ => Err("There is no such day yet, please try again"),
        };

        if !is_err(func) {
            has_selected_day = true;
        } else {
            let answer_one = func(Part::One);
            let answer_two = func(Part::Two);

            println!("************************************************************");
            println!("* Advent of Code: 2022");
            println!("*   Solution for...");
            println!("*     Part One: {}", answer_one);
            println!("*     Part Two: {}", answer_two);
            println!("************************************************************");
        }
    }
}
