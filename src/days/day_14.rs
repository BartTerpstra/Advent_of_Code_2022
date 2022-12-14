use crate::{Output, Part};
use arrayvec::ArrayVec;
use itertools::Position;
use std::borrow::BorrowMut;
use std::cmp::min_by;
use std::error::Error;
use std::ops::Range;
use std::slice::Windows;

const INPUT: &str = include_str!("../../input/14.txt");

pub type Input = CaveCeiling;
pub type Coordinate = (usize, usize);

#[derive(Clone)]
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
        .filter(|x| !x.is_empty())
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
    let height: usize = (max_y + 2 + 1) as usize;
    let width: usize = (max_x + max_y * 2 + 1) as usize;
    let mut slice2d = vec![false; (width * height)];

    //draw rocks into chart
    for rock in atomised {
        let mut first_coord = true;
        let mut previous_coord: Coordinate = (0, 0);
        for coordinate in rock {
            //normalised coordinates
            let x: usize = (coordinate[0]) as usize;
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
    let mut ceil = &mut input.clone();

    //sand drops at 500,0 1 grain at a time until it rests.
    //once sand tries to rest outside of range, terminate

    let drop_coord = (500, 0);
    let mut answer = 0;

    //create grain
    let mut grain = drop_coord.clone();
    //simulate grain
    while in_slice(ceil, grain) {
        let potential_move = down_move(ceil, grain);
        if potential_move.is_some() {
            let new_coord = potential_move.unwrap();
            println!("coord: {},{}", new_coord.0, new_coord.1);
            if new_coord != grain {
                //if it can move down
                grain = potential_move.unwrap();
            } else {
                //settled
                answer += 1;
                ceil.slice2d[to_index(grain, ceil.width)] = true;
                println!("settled: {},{}", grain.0, grain.1);
                grain = drop_coord.clone();
            }
        } else {
            //out of bounds
            break;
        }
    }
    Output::U32(answer)
}

pub fn part2(input: &Input) -> Output {
    let mut ceil = &mut input.clone();
    //sand drops at 500,0 1 grain at a time until it rests.
    //once sand tries to rest outside of range, terminate

    //part 2 addition: (near) infinite floor
    for x in 0..ceil.width {
        ceil.slice2d[x + (ceil.height - 1) * ceil.width] = true;
    }

    let drop_coord = (500, 0);
    let mut answer = 0;
    //create grain
    let mut grain = drop_coord.clone();
    //simulate grain
    while in_slice(ceil, grain) {
        let potential_move = down_move(ceil, grain);
        if potential_move.is_some() {
            let new_coord = potential_move.unwrap();
            println!("coord: {},{}", new_coord.0, new_coord.1);
            if new_coord != grain {
                //if it can move down
                grain = potential_move.unwrap();
            } else {
                //settled
                answer += 1;
                ceil.slice2d[to_index(grain, ceil.width)] = true;
                //choked entrance
                if grain == drop_coord {
                    break;
                }
                grain = drop_coord.clone();
                println!("settled: {},{}", drop_coord.0, drop_coord.1);
            }
        } else {
            //out of bounds
            break;
        }
    }

    //print_ceiling(&ceil);
    Output::U32(answer)
}

fn in_slice(ceil: &CaveCeiling, pos: Coordinate) -> bool {
    return pos.0 < ceil.width && pos.0 >= 0 && pos.1 + 1 < ceil.height;
}

fn down_move(ceil: &CaveCeiling, pos: Coordinate) -> Option<(usize, usize)> {
    assert!(in_slice(ceil, pos));

    if !ceil.slice2d[pos.0 + (pos.1 + 1) * ceil.width] {
        println!("checked filled {},{}", pos.0, pos.1 + 1);
        return Some((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 {
        let left_down = ceil.slice2d[pos.0 - 1 + (pos.1 + 1) * ceil.width];
        if !left_down {
            return Some((pos.0 - 1, pos.1 + 1));
        }
    } else {
        //best move is out of bounds
        return None;
    }
    if pos.0 < ceil.width {
        let right_down = ceil.slice2d[pos.0 + 1 + (pos.1 + 1) * ceil.width];
        if !right_down {
            return Some((pos.0 + 1, pos.1 + 1));
        }
    } else {
        //best move is out of bounds
        return None;
    }
    return Some(pos);
}

//wrong: erronously does wall check
// fn down_move(ceil: &CaveCeiling, pos: Coordinate) -> Option<(usize, usize)> {
//     assert!(in_slice(ceil, pos));
//
//     if !ceil.slice2d[pos.0 + (pos.1 + 1) * ceil.width] {
//         println!("checked filled {},{}", pos.0, pos.1 + 1);
//         return Some((pos.0, pos.1 + 1));
//     }
//     if pos.0 > 0 {
//         let left_down = ceil.slice2d[pos.0 - 1 + (pos.1 + 1) * ceil.width];
//         let left = ceil.slice2d[pos.0 - 1 + (pos.1) * ceil.width];
//         if !left && !left_down {
//             println!(
//                 "checked filled {},{} and {},{}",
//                 pos.0 - 1,
//                 pos.1,
//                 pos.0 - 1,
//                 pos.1 + 1
//             );
//             return Some((pos.0 - 1, pos.1 + 1));
//         }
//     } else {
//         //best move is out of bounds
//         return None;
//     }
//     if pos.0 < ceil.width {
//         let right_down = ceil.slice2d[pos.0 + 1 + (pos.1 + 1) * ceil.width];
//         let right = ceil.slice2d[pos.0 + 1 + (pos.1) * ceil.width];
//         if !right && !right_down {
//             println!(
//                 "checked filled {},{} and {},{}",
//                 pos.0 + 1,
//                 pos.1,
//                 pos.0 + 1,
//                 pos.1 + 1
//             );
//             return Some((pos.0 + 1, pos.1 + 1));
//         }
//     } else {
//         //best move is out of bounds
//         return None;
//     }
//     return Some(pos);
// }

fn to_index(pos: Coordinate, width: usize) -> usize {
    pos.0 + pos.1 * width
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
