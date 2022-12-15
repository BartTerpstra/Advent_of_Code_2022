use crate::helper::Output::U32;
use crate::{Output, Part};
use arrayvec::ArrayVec;
use priority_queue::PriorityQueue;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Index;

const INPUT: &str = include_str!("../../input/12.txt");

//todo class for 2 dimensional map.
const WIDTH: usize = 167;
const HEIGHT: usize = 41;
const SIZE: usize = WIDTH * HEIGHT;
pub type Input = Vec<u8>; //height, weight

#[derive(Clone, Copy, Eq, Hash, Debug)]
struct Node {
    position: Position,
    cost: u32,
    height: u8,
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.cost == other.cost
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug)]
struct Position(u8, u8); //X, Y

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

pub fn read() -> Input {
    let mut ordering: Vec<char> = ('a'..='z').filter(|x| x != &'E' && x != &'S').collect();

    ordering.insert(0, 'S');
    ordering.push('E');
    let ordering = ordering;

    let lookup = ordering
        .into_iter()
        .zip((0..).into_iter())
        .collect::<HashMap<char, u8>>();

    let hill = INPUT
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| *lookup.get(&x).unwrap())
        .collect();

    hill
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

fn index_to_position(index: usize) -> Position {
    let y: u8 = (index / WIDTH) as u8;
    let x: u8 = (index % WIDTH) as u8;
    Position(x, y)
}
fn position_to_index(pos: Position) -> usize {
    (pos.0 as usize + pos.1 as usize * WIDTH) as usize
}

fn valid_neighbours(position: Position) -> Vec<Position> {
    let mut answer = Vec::new();
    if position.0 > 0 {
        answer.push(Position((position.0 - 1), position.1));
    }
    if position.0 < (WIDTH - 1) as u8 {
        answer.push(Position((position.0 + 1), position.1));
    }
    if position.1 > 0 {
        answer.push(Position(position.0, position.1 - 1));
    }
    if position.1 < (HEIGHT - 1) as u8 {
        answer.push(Position(position.0, position.1 + 1));
    }

    answer
}

fn cost_to_priority(cost: u32) -> u32 {
    u32::MAX - cost
}

//too high 529
//too high 522
//wrong 521
pub fn part1(input: &Input) -> Output {
    //do actual dijkstra
    //by floodfilling cost

    let start = {
        let mut answer = Position(u8::MAX, u8::MAX);
        for x in 0..input.len() {
            if input[x] == 0 {
                answer = index_to_position(x);
                break;
            }
        }
        answer
    };

    let end = {
        let mut answer = Position(u8::MAX, u8::MAX);
        for x in 0..input.len() {
            if input[x] == 27 {
                answer = index_to_position(x);
                break;
            }
        }
        answer
    };

    let mut grid: Vec<Node> = {
        let mut grid2 = Vec::new();
        for x in 0..input.len() {
            let position = index_to_position(x);
            grid2.push(Node {
                position,
                height: input[x],
                cost: u32::MAX,
            })
        }
        grid2[position_to_index(start)].cost = 0;
        grid2
    };

    let mut queue: PriorityQueue<Position, u32> = PriorityQueue::new();
    {
        queue.push(start, 0);
    }
    //todo (optional) consider biasing to speedup
    while !queue.is_empty() {
        let considering = queue.pop().unwrap().0;

        let options = valid_neighbours(considering);
        for x in options {
            let height_option = grid[position_to_index(x)].height;
            let height_considering = grid[position_to_index(considering)].height;
            let cost_option = grid[position_to_index(x)].cost;
            let cost_considering = grid[position_to_index(considering)].cost;

            //skip those that go higher or lower than 1
            if !((height_considering + 1) as i32 >= height_option as i32) {
                continue;
            }

            //skip those who have a better score then we want to set them at
            if cost_option <= cost_considering + 1 {
                continue;
            }

            //set their score to considering+1;
            grid[position_to_index(x)].cost = cost_considering + 1;
            queue.push(
                grid[position_to_index(x)].position,
                cost_to_priority(cost_considering + 1),
            );
        }
    }

    if grid[position_to_index(end)].cost != u32::MAX {
        grid_print(&grid, WIDTH);
        return U32(grid[position_to_index(end)].cost);
    }
    grid_print(&grid, WIDTH);
    Output::String("failed to find".to_string())
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}

fn to_letter(num: u8) -> char {
    if num == u8::MAX {
        return '.';
    }
    let look_up: Vec<char> = ('a'..='z').chain(('A'..='Z')).collect();

    let num = num % 52;
    look_up[(num) as usize]
}

fn grid_print(grid: &Vec<Node>, width: usize) {
    for x in 0..grid.len() {
        if x % width == 0 {
            print!("\n");
        }

        print!("{}", to_letter(grid[x].cost as u8));
    }
    print!("\n");
}

pub fn brokenpart1(input: &Input) -> Output {
    // let start = {
    //     let mut answer = Position(u8::MAX, u8::MAX);
    //     for x in 0..input.len() {
    //         if input[x] == 0 {
    //             answer = index_to_position(x);
    //             break;
    //         }
    //     }
    //     answer
    // };
    //
    // let end = {
    //     let mut answer = Position(u8::MAX, u8::MAX);
    //     for x in 0..input.len() {
    //         if input[x] == 25 {
    //             answer = index_to_position(x);
    //             break;
    //         }
    //     }
    //     answer
    // };
    //
    // //todo (optional) create bias (function that increases weight if it moves towards the goal and lowers if it moves away.
    // //todo create priority queue of Trail ordered by cheapest per move. (trail of 10/20 comes before 1/3)
    //
    // let mut queue: PriorityQueue<Trail, u32> = PriorityQueue::new();
    // let head = Trail {
    //     route: Vec::new(),
    //     tail: start,
    //     cost: 0,
    // };
    // let head = head.push(start, 0);
    // let priority = head.priority();
    // queue.push(head, priority);
    //
    // //while trail not reached destination
    // while !queue.is_empty() {
    //     //pick head of queue
    //     let considering = queue.pop().unwrap().0;
    //
    //     //add all options that don't loop back on itself to queue as new trails
    //     let options = valid_neighbours(considering.tail);
    //
    //     options
    //         .iter()
    //         .filter(/*ledge*/ |x| {
    //             input[position_to_index(**x)].abs_diff(input[position_to_index(considering.tail)])
    //                 <= 1
    //         })
    //         .filter(
    //             /*prevent loops (efficiency only. might slow instead)*/
    //             |x| !considering.route.contains(x),
    //         )
    //         .map(|x| {
    //             considering.push(*x, 1 /*todo bias here*/)
    //         })
    //         .for_each(|x| {
    //             let mut priority = x.priority();
    //             if input[position_to_index(x.tail)] == input[position_to_index(end)] {
    //                 println!("WINNER: {:?}", considering.route);
    //                 priority = u32::MAX;
    //             }
    //             queue.push(x, priority);
    //         });
    //     if queue.peek().unwrap().0.tail == end {
    //         return Output::U32(queue.peek().unwrap().0.route.len() as u32 + 1);
    //     }
    // }
    // //endwhile
    Output::String("failed to find".to_string())
}

//
// #[derive(Eq)]
// struct Trail {
//     route: Vec<Position>,
//     tail: Position,
//     cost: u32,
// }
//
// impl Hash for Trail {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.tail.hash(state);
//         self.cost.hash(state);
//     }
// }
//
// impl PartialEq<Self> for Trail {
//     fn eq(&self, other: &Self) -> bool {
//         self.tail == other.tail
//     }
// }
//
// impl Trail {
//     fn get_normalised_cost(&self) -> u32 {
//         if self.cost == 0 {
//             return 0;
//         };
//         self.route.len() as u32 / self.cost
//     }
//
//     fn priority(&self) -> u32 {
//         u32::MAX - self.get_normalised_cost()
//     }
//
//     fn push(&self, route_step: Position, step_price: u32) -> Trail {
//         let mut route = self.route.to_vec();
//         route.push(route_step);
//         Trail {
//             route,
//             tail: route_step,
//             cost: self.cost + step_price,
//         }
//     }
// }
