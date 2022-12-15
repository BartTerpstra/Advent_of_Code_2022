use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../../input/8.txt");
const FOREST_WIDTH: usize = 99;
const FOREST_HEIGHT: usize = 99;
const FOREST_AREA: usize = FOREST_WIDTH * FOREST_HEIGHT;

#[derive(Debug)]
pub struct Tree {
    height: u8,
    spotted_from_edge: bool,
}
pub type Input = ArrayVec<Tree, FOREST_AREA>;

pub fn read() -> Input {
    let mut answer: Input = ArrayVec::new();
    for x in INPUT.chars() {
        if x == '\n' {
            continue;
        };
        let height = match x {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("input validation error!"),
        };

        answer.push(Tree {
            height,
            spotted_from_edge: false,
        });
    }
    return answer;
}

/**The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

30373
25512
65332
33549
35390

Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it.
Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge,
there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

    The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
    The top-middle 5 is visible from the top and right.
    The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
    The left-middle 5 is visible, but only from the right.
    The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
    The right-middle 3 is visible from the right.
    In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?
**/
pub fn run(part: Part) -> Output {
    let mut input1 = read();
    let input2 = read();
    match part {
        Part::One => part1(&mut input1),
        Part::Two => part2(&input2),
    }
}

/**
strategy:
mark all trees on the edge as visible,
for each cardinal direction (e,w,s,n) check every line leading from that edge
starting with the internal trees check one close to the edge**/
pub fn part1(input: &mut Input) -> Output {
    //todo higher answer than 572
    //todo rather than find edge, check entire row, even if there is a dip in the heights

    for x in 0..FOREST_WIDTH {
        input[x].spotted_from_edge = true; //the first row:0,1,2,3, etc
        input[x + (FOREST_WIDTH - 1) * FOREST_HEIGHT].spotted_from_edge = true; //the last row: 99*98
    }
    for y in 0..FOREST_HEIGHT {
        input[y * FOREST_WIDTH].spotted_from_edge = true; //the west side: 0,99,198, etc
        input[y * FOREST_WIDTH + FOREST_WIDTH - 1].spotted_from_edge = true; //the east side: 98, 197, etc
    }

    //for north edge inners
    //for every inner
    //either check all of them or until you have found a tree with max height.
    for x in 1..FOREST_WIDTH - 1 {
        let mut found_height: u8 = input[x].height;
        let mut depth = 1;
        while found_height != 9 && depth < FOREST_WIDTH {
            let inner_index = x + (FOREST_WIDTH * depth);
            if can_be_seen(found_height, &input[inner_index]) {
                input[inner_index].spotted_from_edge = true;
                found_height = input[inner_index].height;
            }
            depth += 1
        }
    }

    //for south edge inners
    for x in 1..FOREST_WIDTH - 1 {
        let first_index_last_row = FOREST_WIDTH * (FOREST_HEIGHT - 1);
        let mut found_height = input[x + first_index_last_row].height;
        let mut depth = 1;
        while found_height != 9 && depth < FOREST_WIDTH {
            let inner_index = x + first_index_last_row - (FOREST_WIDTH * (depth - 1));
            if can_be_seen(found_height, &input[inner_index]) {
                input[inner_index].spotted_from_edge = true;
                found_height = input[inner_index].height;
            }
            depth += 1;
        }
    }

    //for west edge inners
    for x in 1..FOREST_HEIGHT - 1 {
        let mut found_height = input[x * FOREST_WIDTH].height;
        let mut depth = 1;
        while found_height != 9 && depth < 99 {
            let inner_index = x * FOREST_WIDTH + depth;
            if can_be_seen(found_height, &input[inner_index]) {
                input[inner_index].spotted_from_edge = true;
                found_height = input[inner_index].height;
            }
            depth += 1;
        }
    }

    //for east edge inners
    for x in 1..FOREST_HEIGHT - 1 {
        let upper_right_index = FOREST_WIDTH - 1;
        let mut found_height = input[x * FOREST_WIDTH + upper_right_index].height;
        let mut depth = 1;
        while found_height != 9 && depth < 99 {
            let inner_index = x * FOREST_WIDTH + upper_right_index - depth;
            if can_be_seen(found_height, &input[inner_index]) {
                input[inner_index].spotted_from_edge = true;
                found_height = input[inner_index].height;
            }
            depth += 1;
        }
    }

    let answer: u32 = input
        .iter()
        .map(|x| x.spotted_from_edge)
        .filter(|x| *x)
        .count() as u32;

    Output::U32(answer)
}

//too high        220320
//too low         168000
//                201600
pub fn part2(input: &Input) -> Output {
    let mut highest_found = 0;
    for x in 0..FOREST_WIDTH {
        for y in 0..FOREST_HEIGHT {
            let mut up: i32 = y as i32 - 1;
            let mut down = y + 1;
            let mut left: i32 = x as i32 - 1;
            let mut right = x + 1;

            let mut seen_up = 0;
            let mut seen_down = 0;
            let mut seen_left = 0;
            let mut seen_right = 0;

            let center = &input[to_index(x, y)];

            while up >= 0 {
                let checking = &input[to_index(x, up as usize)];
                seen_up += 1;

                if checking.height >= center.height {
                    break;
                }

                if up == 0 {
                    break;
                }
                up -= 1;
            }

            while down < FOREST_HEIGHT {
                let checking = &input[to_index(x, down)];
                seen_down += 1;
                if checking.height >= center.height {
                    break;
                }
                down += 1;
            }
            while left >= 0 {
                let checking = &input[to_index(left as usize, y)];
                seen_left += 1;
                if checking.height >= center.height {
                    break;
                }
                if left == 0 {
                    break;
                }
                left -= 1;
            }

            while right < FOREST_WIDTH {
                let checking = &input[to_index(right, y)];
                seen_right += 1;
                if checking.height >= center.height {
                    break;
                }
                right += 1;
            }

            let total: u32 = seen_up * seen_down * seen_left * seen_right;
            if total > highest_found {
                highest_found = total;
            }
        }
    }
    Output::U32(highest_found)
}

fn to_index(x: usize, y: usize) -> usize {
    x + y * FOREST_WIDTH
}

fn can_be_seen(outer: u8, inner: &Tree) -> bool {
    outer < inner.height
}
