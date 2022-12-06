use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../input/6.txt");

pub type Input = Vec<char>; //todo example, do change

pub fn read() -> Input {
    //TODO basically just string slice INPUT by line and select and convert to correct type.
    INPUT.chars().collect()
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

pub fn part1(input: &Input) -> Output {
    let mut count: u32 = 0;
    let mut window: ArrayVec<char, 4> = ArrayVec::new();

    for x in input.iter().take(4) {
        window.push(*x);
        count += 1;
    }
    if is_unique(&window) {
        println!("{:?}", window);
        return Output::U8(4);
    }
    for x in input.iter().skip(4) {
        window.pop_at(0);
        window.push(*x);
        count += 1;
        if is_unique(&window) {
            return Output::U32(count);
        }
    }

    Output::U32(count)
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}

fn is_unique(window: &ArrayVec<char, 4>) -> bool {
    for x in 0..3 {
        for y in (x + 1)..4 {
            if window[x] == window[y] {
                return false;
            }
        }
    }
    return true;
}
