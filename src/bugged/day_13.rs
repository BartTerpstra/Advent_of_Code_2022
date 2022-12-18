// use crate::helper::Output::String;
// use crate::{Output, Part};
// use arrayvec::{ArrayString, ArrayVec};
// use std::ops::Add;
// use std::string;
//
// const INPUT: &str = include_str!("../../input/13_test.txt");
//
// pub type Input = Vec<(PacketItem, PacketItem)>;
//
// enum PacketItem {
//     List(Vec<PacketItem>),
//     Value(u8),
// }
//
// fn get_value(item: &PacketItem) -> u8 {
//     match item {
//         PacketItem::List(x) => return get_value(&x[0]),
//         PacketItem::Value(x) => return *x,
//     }
//     panic!("unexpected state")
// }
//
// pub fn read() -> Input {
//     INPUT
//         .split("\n\n")
//         .map(|x| {
//             let mut first = true;
//             let mut answer = ("", "");
//             let split = x.lines().map(|x| {
//                 if first {
//                     first = false;
//                     answer.0 = x;
//                 } else {
//                     answer.1 = x;
//                 }
//             });
//             answer
//         })
//         .map(|x| (to_packet(x.0), to_packet(x.1)))
//         .collect()
// }
//
// pub fn run(part: Part) -> Output {
//     let input = read();
//     match part {
//         Part::One => part1(&input),
//         Part::Two => part2(&input),
//     }
// }
//
// pub fn part1(input: &Input) -> Output {
//     let mut answers: Vec<u16> = Vec::new();
//     let mut index: u16 = 1;
//     for x in input {
//         let left = &x.0;
//         let right = &x.1;
//
//         let th = match left {
//             PacketItem::List(left) => {
//                 match right {
//                     PacketItem::List(right) => {
//                         let mut answer = true;
//                         for index in 0..left.len() {
//                             if index > right.len() {
//                                 answer = false; //right side ran out while left side had items
//                                 break;
//                             }
//                             if get_value(&right[index]) < get_value(&left[index]) {
//                                 answer = false; //right side was larger
//                                 break;
//                             } else if get_value(&right[index]) > get_value(&left[index]) {
//                                 break; //left side was smaller
//                             }
//                         }
//                         answer //left items ran out
//                     }
//                     PacketItem::Value(right) => compare_list_and_num(true, left, right),
//                 }
//             }
//             PacketItem::Value(left) => match right {
//                 PacketItem::List(right) => compare_list_and_num(false, right, left),
//                 PacketItem::Value(right) => (left <= right),
//             },
//         };
//         if th {
//             answers.push(index);
//         }
//
//         index += 1;
//     }
//
//     let answer = answers.iter().fold("".to_string(), |x, y| {
//         x.add(y.to_string().as_str()).add(", ")
//     });
//     Output::String(answer)
// }
//
// pub fn part2(input: &Input) -> Output {
//     Output::U32(1)
// }
//
// /** returns true if
// left >= right && list is the left hand
// OR
// right >= left && list is the right hand
// */
// fn compare_list_and_num(is_list_left: bool, list: &Vec<PacketItem>, num: &u8) -> bool {
//     let answer = false;
//     let answer = match &list[0] {
//         PacketItem::List(x) => {
//             panic!("unexpected list in comparison")
//         }
//         PacketItem::Value(list_item) => list_item >= num,
//     };
//
//     is_list_left == answer
// }
//
// fn to_packet(string: &str) -> PacketItem {
//     if string.is_empty() {
//         return empty();
//     }
//     if string == "[]" {
//         return empty();
//     };
//
//     assert!(string.len() >= 3, "string too short");
//     let mut packet = Vec::new();
//
//     let mut num = "".to_string();
//     for x in string.chars() {
//         match x {
//             '[' => packet.push(to_packet(&string[1..string.len()])),
//             ']' => {
//                 if !num.is_empty() {
//                     packet.push(PacketItem::Value(num.parse::<u8>().unwrap()))
//                 }
//                 return PacketItem::List(packet);
//             }
//             ',' => {
//                 packet.push(PacketItem::Value(num.parse::<u8>().unwrap()));
//                 num = "".to_string()
//             }
//             x => {
//                 num.push(x);
//             }
//         }
//     }
//
//     panic!("unreachable 1")
// }
//
// fn empty() -> PacketItem {
//     return PacketItem::List(Vec::new());
// }
