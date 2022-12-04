use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../input/4.txt");

pub type Input = ArrayVec<&'static str, 1000>; //todo example, do change

pub fn read() -> Input {
    INPUT.split('\n').collect()
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

//too high 871
pub fn part1(input: &Input) -> Output {
    let mut sum = 0;
    for x in input {
        let ranges: Vec<&str> = x.split(',').collect();
        assert_eq!(ranges.len(), 2);

        let left_range: Vec<&str> = ranges.get(0).unwrap().split('-').collect();

        let right_range: Vec<&str> = ranges.get(1).unwrap().split('-').collect();

        let ll = left_range.get(0).unwrap().parse::<u8>().unwrap();
        let lr = left_range.get(1).unwrap().parse::<u8>().unwrap();
        let rl = right_range.get(0).unwrap().parse::<u8>().unwrap();
        let rr = right_range.get(1).unwrap().parse::<u8>().unwrap();

        print!("list: {:?} ", [ll, lr, rl, rr,]);
        println!(
            "result: {}",
            is_either_one_superset_or_equal(ll, lr, rl, rr)
        );

        if is_either_one_superset_or_equal(ll, lr, rl, rr) {
            sum += 1;
        }
    }
    Output::U32(sum)
}

//too low 762
pub fn part2(input: &Input) -> Output {
    let mut sum = 0;
    for x in input {
        let ranges: Vec<&str> = x.split(',').collect();
        assert_eq!(ranges.len(), 2);

        let left_range: Vec<&str> = ranges.get(0).unwrap().split('-').collect();

        let right_range: Vec<&str> = ranges.get(1).unwrap().split('-').collect();

        let ll = left_range.get(0).unwrap().parse::<u8>().unwrap();
        let lr = left_range.get(1).unwrap().parse::<u8>().unwrap();
        let rl = right_range.get(0).unwrap().parse::<u8>().unwrap();
        let rr = right_range.get(1).unwrap().parse::<u8>().unwrap();

        // print!("list: {:?} ", [ll, lr, r762l, rr,]);
        // println!(
        //     "result: {}",
        //     is_either_one_superset_or_equal(ll, lr, rl, rr)
        // );

        if do_overlap(ll, lr, rl, rr) {
            sum += 1;
        }
    }
    Output::U32(sum)
}

//does either one fit entirely inside inclusive bound of another?
fn is_either_one_superset_or_equal(ll: u8, lr: u8, rl: u8, rr: u8) -> bool {
    (ll <= rl && lr >= rr) || (rl <= ll && rr >= lr)
}

//is either hand of the right range inside the bound of the left range?
fn do_overlap(ll: u8, lr: u8, rl: u8, rr: u8) -> bool {
    (rl >= ll && rl <= lr) || (rr >= ll && rr <= lr) || (rl <= ll && rr >= lr)
}
