use crate::{Output, Part};
use arrayvec::ArrayVec;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Index;

const INPUT: &str = include_str!("../input/12_test.txt");

//todo class for 2 dimensional map.
const WIDTH: usize = 168;
const HEIGHT: usize = 41;
const SIZE: usize = WIDTH * HEIGHT;
pub type Input = Vec<u8>;

struct Trail {
    route: Vec<(u8, u8)>,
    tail: (u8, u8),
    cost: u32,
}

pub fn read() -> Input {
    let mut ordering: Vec<char> = ('a'..='z')
        .chain(('A'..='Z'))
        .filter(|x| x != &'E' && x != &'S')
        .collect();
    println!("order:{:?} ", &ordering);

    ordering.insert(0, 'S');
    ordering.push('E');
    let ordering = ordering;
    println!("order:{:?} ", &ordering);

    let lookup = ordering
        .into_iter()
        .zip((0..).into_iter())
        .collect::<HashMap<char, u8>>();

    println!("lookup:{:?} ", &lookup);

    let hill = INPUT
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| *lookup.get(&x).unwrap())
        .collect();

    println!("hill:{:?} ", hill);
    hill
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

fn index_to_position(index: usize) -> (u8, u8) {
    let y: u8 = (index / WIDTH) as u8;
    let x: u8 = (index % WIDTH) as u8;
    (x, y)
}

pub fn part1(input: &Input) -> Output {
    let start: (u8, u8) = {
        let mut answer = (u8::MAX, u8::MAX);
        for x in 0..input.len() {
            if input[x] == 0 {
                answer = index_to_position(x);
                break;
            }
        }
        answer
    };

    let end: (u8, u8) = {
        let mut answer = (u8::MAX, u8::MAX);
        for x in 0..input.len() {
            if input[x] == 24 {
                answer = index_to_position(x);
                break;
            }
        }
        answer
    };

    //todo (optional) create bias (function that increases weight if it moves towards the goal and lowers if it moves away.
    // let queue = priority_queue::new();
    //todo create priority queue of Trail ordered by cheapest per move. (trail of 10/20 comes before 1/3)
    //while trail not reached destination
    //pick head of queue
    //take best option
    //break and return if end
    //else add to queue
    //endwhile

    Output::U32(0)
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}
