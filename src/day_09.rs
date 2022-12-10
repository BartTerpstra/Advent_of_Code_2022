use crate::day_09::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::{Output, Part};
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input/9.txt");

pub type Input = Vec<Operation>;

#[derive(Eq, Hash, Clone, Copy)]
struct RopeEnd {
    x: i32,
    y: i32,
}

impl PartialEq<Self> for RopeEnd {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Operation {
    direction: Direction,
    amount: u8,
}

enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

fn to_direction(char: char) -> Direction {
    match char {
        'L' => LEFT,
        'R' => RIGHT,
        'U' => UP,
        'D' => DOWN,
        _ => panic!("input processing error. no such direction"),
    }
}

fn nature_abhors_a_vacuum(head: &RopeEnd, tail: &mut RopeEnd) {
    if does_border(head, tail) {
        return;
    } else {
        if head.x > tail.x {
            tail.x += 1;
        } else {
            tail.x -= 1;
        }
        if head.y > tail.y {
            tail.y += 1;
        } else {
            tail.y -= 1;
        }
    }
}

/** anything with 2 difference gets rejected  
*    11|01|11
*    10|00|10
*    11|01|11
*/
fn does_border(one: &RopeEnd, two: &RopeEnd) -> bool {
    one.x.abs_diff(two.x) <= 1 && one.y.abs_diff(two.y) <= 1
}

pub fn read() -> Input {
    let lines: Vec<&str> = INPUT.lines().collect();
    let operations: Vec<Operation> = lines
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let v: Vec<&str> = x.split(' ').take(2).collect();
            let op = Operation {
                direction: to_direction(v[0].chars().take(1).exactly_one().unwrap()),
                amount: v[1].parse().unwrap(),
            };

            op
        })
        .collect();

    operations
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

//too low 5027
pub fn part1(input: &Input) -> Output {
    assert_eq!(input.len(), 2000);

    let mut head = RopeEnd { x: 0, y: 0 };
    let mut tail = RopeEnd { x: 0, y: 0 };
    let mut visited = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    visited.insert(RopeEnd { x: 0, y: 0 });

    for x in input {
        let amount = x.amount;
        for _ in 0..amount {
            match x.direction {
                UP => head.y += 1,
                LEFT => head.x -= 1,
                DOWN => head.y -= 1,
                RIGHT => head.x += 1,
            }
            nature_abhors_a_vacuum(&head, &mut tail);
            visited.insert(tail);
        }
        if tail.x.abs() > max_x {
            max_x = tail.x;
        }
        if tail.y.abs() > max_y {
            max_y = tail.y;
        }
    }

    println!("max x: {}", max_x);
    println!("max y: {}", max_y);

    Output::U32(visited.len() as u32)
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}
