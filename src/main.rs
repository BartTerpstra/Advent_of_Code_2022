extern crate core;

mod Grid;
mod days;
mod helper;

use helper::*;

fn main() {
    //todo cli menu
    //todo timing
    //todo (nice to have) automated puzzle downloading and answer checking

    println!("Welcome to Advent of Code 2022 solver");

    /* run all*/
    // let all_days = days::all();
    //
    // let mut postion: u8 = 1;
    // for day in all_days {
    //     let answer_one = day(Part::One);
    //     let answer_two = day(Part::Two);
    //
    //     println!("************************************************************");
    //     println!("* Advent of Code: 2022");
    //     println!("*   Solution for day {}", postion);
    //     println!("*     Part One: {}", answer_one);
    //     println!("*     Part Two: {}", answer_two);
    //     println!("************************************************************");
    //
    //     postion += 1;
    // }

    //debug: run 1
    let num: usize = 1;
    let day = days::all()[num - 1];
    let answer_one = day(Part::One);
    let answer_two = day(Part::Two);

    println!("************************************************************");
    println!("* Advent of Code: 2022");
    println!("*   Solution for day {}", num);
    println!("*     Part One: {}", answer_one);
    println!("*     Part Two: {}", answer_two);
    println!("************************************************************");

    /* todo menu*/
    // let mut func: fn(Part) -> Output;
    //
    // let items: Vec<String> = (1..25).map(|x: i32| x.to_string()).collect::<Vec<String>>();
    // let func = match input {
    //     _ => day_12::run,
    // };
}
