use crate::{Output, Part};
use arrayvec::ArrayVec;
use std::cmp::min_by;
use std::error::Error;
use std::ops::Range;
use std::slice::Windows;

const INPUT: &str = include_str!("../../input/14_test.txt");

pub type Input = CaveCeiling;

struct CaveCeiling {
    normalised_by: usize,
    width: usize,
    height: usize,
    slice2d: Vec<bool>,
}

pub fn read() -> Input {
    type T = u16;
    //split by line (a rock)
    //split by " -> " (coordinates)
    //split by "," (x and y)
    let atomised: Vec<Vec<Vec<T>>> = INPUT
        .split("\n")
        .map(|x| {
            x.split(" -> ")
                .map(|x| x.split(",").map(|x| x.parse::<u16>().unwrap()).collect())
                .collect()
        })
        .collect();

    //normalisation values
    let mut max_x = T::MIN;
    let mut max_y = T::MIN;
    let mut min_x = T::MAX;
    let min_y = 0;

    for rock in &atomised {
        for coordinate in rock {
            assert_eq!(
                coordinate.len(),
                2,
                "coordinate contained wrong amount of int"
            );
            let x = coordinate[0];
            let y = coordinate[1];

            //normalisation values
            if x > max_x {
                max_x = x;
            }
            if x < min_x {
                min_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }

    //hint: minimum values are normalisation values
    //y does not have to be normalised
    let width: usize = (max_x - min_x + 1) as usize;
    let height: usize = (max_y + 1) as usize;
    let mut slice2d = vec![false; (width * height)];

    for rock in atomised {
        let mut first_coord = true;
        let mut previous_coord: (usize, usize) = (0, 0);
        for coordinate in rock {
            //normalised coordinates
            let x: usize = (coordinate[0] - min_x) as usize;
            let y: usize = (coordinate[1]) as usize;

            //set rocks into ceiling
            if first_coord {
                let index: usize = (x + y * width) as usize;
                slice2d[index] = true;
                first_coord = false;
                previous_coord = (x, y);
            } else {
                if previous_coord.0 != x {
                    let range = match previous_coord.0 > x {
                        true => (x, previous_coord.0),
                        false => (previous_coord.0, x),
                    };

                    for target_x in range.0..=range.1 {
                        let index: usize = (target_x + y * width) as usize;
                        slice2d[index] = true;
                    }
                } else {
                    let range = match previous_coord.1 > y {
                        true => (y, previous_coord.1),
                        false => (previous_coord.1, y),
                    };

                    for target_y in range.0..=range.1 {
                        let index: usize = (x + target_y * width) as usize;
                        slice2d[index] = true;
                    }
                }

                previous_coord = (x, y);
            }
        }
    }

    CaveCeiling {
        normalised_by: min_x as usize,
        width,
        height,
        slice2d,
    }
}

/**https://adventofcode.com/2022/day/14*/
pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

/** How many units of sand come to rest before sand starts flowing into the abyss below?*/
pub fn part1(input: &Input) -> Output {
    print_ceiling(input);
    //todo simulate sand
    //sand drops at 500,0 1 grain at a time until it rests.
    //once sand tries to rest outside of range, terminate

    let drop_coord = (500 - input.normalised_by, 0);
    let answer = 0;
    let dropped_out_of_bounds = false;
    while !dropped_out_of_bounds {
        //create grain
        let grain = drop_coord;
        //simulate grain
        //am i out of bounds?
        //return
        //continue
        //can i move down?
        //move goto
        //settle next grain
    }
    Output::U32(answer)
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}

fn print_ceiling(ceil: &CaveCeiling) {
    for index in 0..ceil.slice2d.len() {
        if index % ceil.width == 0 {
            print!("{} ", index / ceil.width)
        }
        if ceil.slice2d[index] {
            print!("#");
        } else {
            print!(".");
        }
        if index % ceil.width == (ceil.width - 1) {
            print!("\n");
        }
    }
}
