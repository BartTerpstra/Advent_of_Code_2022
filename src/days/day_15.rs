use crate::Grid::{BinaryState, Coordinate, Grid, SignedCoord};
use crate::{Output, Part};
use arrayvec::ArrayVec;
use itertools::{Itertools, Position};
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/15_test.txt");

type HasBeenRuledOut = BinaryState;
pub type Input = Grid<HasBeenRuledOut>; //grid of "has been ruled out"

pub fn read() -> Input {
    let atoms: Vec<Vec<Vec<i32>>> = INPUT
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.strip_prefix("Sensor at ")
                .unwrap()
                .split(": closest beacon is at ")
                .map(|x| {
                    x.split(", ")
                        .map(|x| x.get(2..).unwrap().parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut sensor_beacon_list = Vec::new();
    let mut coords: Vec<(SignedCoord, SignedCoord)> = Vec::new();
    for x in &atoms {
        let mut is_beacon = false;
        for y in x {
            assert_eq!(y.len(), 2, "coord without enough values");
            coords.push((y[0], y[1]));
            sensor_beacon_list.push((is_beacon, y[0], y[1]));
            is_beacon = !is_beacon;
        }
    }

    //state: has been ruled out
    let mut grid = Grid::new_dynamic(coords, 0, HasBeenRuledOut::False);

    let normalised_coords: Vec<(bool, Coordinate)> = sensor_beacon_list
        .iter()
        .map(|x| (x.0, grid.normalise_to_grid((x.1, x.2))))
        .collect();

    //for every pair of beacon sensor, fill taxicab radius with true
    for index in 0..normalised_coords.len() / 2 {
        assert_ne!(normalised_coords[index].0, normalised_coords[index + 1].0);
        let sensor = normalised_coords[index * 2].1;
        let beacon = normalised_coords[index * 2 + 1].1;
        let radius = sensor.taxicab_distance(beacon);

        let mut to_mark = HashSet::new();
        to_mark.insert(sensor);
        for countdown in 0..radius {
            let mut temp: Vec<Coordinate> = Vec::new();
            for x in &to_mark {
                let mut y = grid.in_grid_neighbours(*x);
                temp.append(&mut y)
            }
            temp.iter().for_each(|x| {
                to_mark.insert(*x);
            })
        }

        for x in to_mark {
            if x != beacon {
                grid.setc(x, BinaryState::True);
            }
        }
    }

    grid.print();
    grid
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

pub fn part1(input: &Input) -> Output {
    let start_of_row = Coordinate { x: 0, y: 10 };
    let mut count = 0;
    let mut pointer = start_of_row;
    while pointer.x < input.width {
        if *input.getc(pointer) == BinaryState::True {
            count += 1;
        }
        pointer.x += 1
    }

    Output::U32(count)
}

pub fn part2(input: &Input) -> Output {
    Output::U32(0)
}
