use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../../input/5.txt");

pub type Input = &'static str; //todo example, do change

pub fn read() -> Input {
    //TODO basically just string slice INPUT by line and select and convert to correct type.
    INPUT
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

pub fn part1(input: &Input) -> Output {
    //create array of lists of items representing the dock
    let mut grid: ArrayVec<Vec<char>, 9> = ArrayVec::new();
    for _ in 0..9 {
        grid.push(Vec::new());
    }

    // for each line, check every container space on that heigh. save it if there is something
    for x in input.lines().take(8) {
        //IF grid
        let charline: Vec<char> = x.chars().collect();
        for x in 0..9 {
            let char = charline[x * 4 + 1];
            if char != ' ' {
                grid[x].push(char);
            }
        }
    }

    grid.iter_mut().for_each(|c| c.reverse());

    // println!("grid: \n {:?}", grid);

    let mut instructions: Vec<(u8, u8, u8)> = Vec::new();

    for instruction in input.lines().skip(10) {
        if instruction.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = instruction.split(' ').collect();
        let amount: u8 = tokens[1].parse::<u8>().unwrap();
        let from: u8 = tokens[3].parse::<u8>().unwrap();
        let to: u8 = tokens[5].parse::<u8>().unwrap();

        instructions.push((amount, from, to));
    }
    // println!("instructions: \n{:?}", instructions);

    for instruction in instructions {
        //illegal because it's still unstable
        //let items_to_move = &grid[instruction.1 as usize].take(instruction.0..);

        for _ in 0..instruction.0 {
            let item_to_move = grid[(instruction.1 - 1) as usize].pop().unwrap();

            grid[(instruction.2 - 1) as usize].push(item_to_move);
        }
    }
    // println!("final state: {:?}", grid);

    let answer: String = grid.iter().fold(String::new(), |mut x, y| {
        x.push(*y.last().unwrap());
        return x;
    });

    Output::String(answer)
}

pub fn part2(input: &Input) -> Output {
    //create array of lists of items representing the dock
    let mut grid: ArrayVec<Vec<char>, 9> = ArrayVec::new();
    for _ in 0..9 {
        grid.push(Vec::new());
    }

    // for each line, check every container space on that height. save it if there is something
    for x in input.lines().take(8) {
        //IF grid
        let charline: Vec<char> = x.chars().collect();
        for x in 0..9 {
            let char = charline[x * 4 + 1];
            if char != ' ' {
                grid[x].push(char);
            }
        }
    }

    grid.iter_mut().for_each(|c| c.reverse());

    // println!("grid: \n {:?}", grid);

    let mut instructions: Vec<(u8, u8, u8)> = Vec::new();

    for instruction in input.lines().skip(10) {
        if instruction.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = instruction.split(' ').collect();
        let amount: u8 = tokens[1].parse::<u8>().unwrap();
        let from: u8 = tokens[3].parse::<u8>().unwrap();
        let to: u8 = tokens[5].parse::<u8>().unwrap();

        instructions.push((amount, from, to));
    }
    // println!("instructions: \n{:?}", instructions);

    for instruction in instructions {
        //illegal because it's still unstable
        //let items_to_move = &grid[instruction.1 as usize].take(instruction.0..);

        let mut items_to_move: Vec<char> = Vec::new();
        for _ in 0..instruction.0 {
            items_to_move.push(grid[(instruction.1 - 1) as usize].pop().unwrap());
        }
        for _ in 0..instruction.0 {
            grid[(instruction.2 - 1) as usize].push(items_to_move.pop().unwrap());
        }
    }
    // println!("final state: {:?}", grid);

    let answer: String = grid.iter().fold(String::new(), |mut x, y| {
        x.push(*y.last().unwrap());
        return x;
    });

    Output::String(answer)
}
