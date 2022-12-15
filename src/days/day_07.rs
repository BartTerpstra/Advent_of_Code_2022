use crate::{Output, Part};
use arrayvec::ArrayVec;

const INPUT: &str = include_str!("../../input/7.txt");

pub type Input = ArrayVec<CLI, 899>; //todo example, do change

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CLI {
    cd(CLI_Directory),
    ls,
    file(u32, String),
    directory(CLI_Directory),
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum CLI_Directory {
    dir(String),
    dir_back,
    root,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
    is_directory: bool,
    children: Vec<Address>,
    parent: Address,
    index: Address,
}

#[derive(Clone, Copy, Debug)]
struct Address {
    index: usize,
}

fn new_file(name: String, size: u32, parent: Address, memory: &mut ArrayVec<File, 429>) {
    //increase parent folder size
    recursive_size_increase(parent, size, memory);
    let new_file = File {
        name,
        size,
        is_directory: false,
        children: vec![],
        parent,
        index: Address {
            index: memory.len(),
        },
    };
    memory
        .get_mut(parent.index)
        .unwrap()
        .children
        .push(new_file.index);
    memory.push(new_file);
}

fn new_dir(name: String, parent: Address, memory: &mut ArrayVec<File, 429>) {
    let new_dir = File {
        name,
        size: 0,
        is_directory: true,
        children: vec![],
        parent,
        index: Address {
            index: memory.len(),
        },
    };
    memory
        .get_mut(parent.index)
        .unwrap()
        .children
        .push(new_dir.index);
    memory.push(new_dir);
}

fn recursive_size_increase(initial_folder: Address, size: u32, mem: &mut ArrayVec<File, 429>) {
    if initial_folder.index == usize::MAX {
        return;
    }
    let curr_folder = mem.get_mut(initial_folder.index).unwrap();
    curr_folder.size += size;
    recursive_size_increase(curr_folder.parent, size, mem)
}

pub fn read() -> Input {
    //read lines as CLI in and output
    let list: Vec<Vec<&str>> = INPUT.split('\n').map(|x| x.split(' ').collect()).collect();

    let mut answer: ArrayVec<CLI, 899> = ArrayVec::new();
    for x in list {
        let addition = match x[0] {
            "$" => match x[1] {
                "cd" => CLI::cd(match x[2] {
                    "/" => CLI_Directory::root,
                    ".." => CLI_Directory::dir_back,
                    _ => CLI_Directory::dir(x[2].to_string()),
                }),
                "ls" => CLI::ls,
                _ => {
                    panic!("error decoding cli tree")
                }
            },
            "dir" => CLI::directory(CLI_Directory::dir(x[1].to_string())),
            _ => CLI::file(x[0].parse::<u32>().unwrap(), x[1].to_string()),
        };
        answer.push(addition);
    }

    return answer;
}

pub fn run(part: Part) -> Output {
    let input = read();
    match part {
        Part::One => part1(&input),
        Part::Two => part2(&input),
    }
}

pub fn part1(input: &Input) -> Output {
    //build file structure
    //breaking assumption: every folder is only visited once
    let mut root = File {
        name: "/".to_string(),
        size: 0,
        is_directory: true,
        children: vec![],
        parent: Address { index: usize::MAX },
        index: Address { index: 0 },
    };

    let mut memory: ArrayVec<File, 429> = ArrayVec::new();
    memory.push(root);

    let mut current_directory: Address = Address { index: 0 };

    //construct the tree in memory
    for x in input {
        match x {
            CLI::cd(d) => match d {
                CLI_Directory::dir(name) => {
                    current_directory = memory
                        .get(current_directory.index)
                        .unwrap()
                        .children
                        .iter()
                        .map(|x| memory.get(x.index).unwrap())
                        .find(|c| c.name == *name)
                        .unwrap()
                        .index
                }
                CLI_Directory::dir_back => {
                    current_directory = memory.get(current_directory.index).unwrap().parent
                }
                CLI_Directory::root => current_directory = Address { index: 0 },
            },
            CLI::ls => {} //do nothing
            CLI::file(size, name) => {
                if already_found(&name.to_string(), current_directory, &memory) {
                    continue;
                }
                new_file(name.to_string(), *size, current_directory, &mut memory);
            }
            CLI::directory(d) => {
                let name = match d {
                    CLI_Directory::dir(d) => d.to_string(),
                    CLI_Directory::dir_back => panic!("file system contains loop"), //these could be solved
                    CLI_Directory::root => panic!("file system contains loop"), //these could be solved
                };
                if already_found(&name, current_directory, &memory) {
                    continue;
                }
                new_dir(name, current_directory, &mut memory);
            }
        }
    }

    let answer: u32 = memory
        .iter()
        .filter(|x| x.is_directory)
        .filter(|x| x.size <= 100000)
        .map(|x| x.size)
        .sum();

    //too high 1565323
    //1427048
    Output::U32(answer)
}

pub fn part2(input: &Input) -> Output {
    //build file structure
    //breaking assumption: every folder is only visited once
    let mut root = File {
        name: "/".to_string(),
        size: 0,
        is_directory: true,
        children: vec![],
        parent: Address { index: usize::MAX },
        index: Address { index: 0 },
    };

    let mut memory: ArrayVec<File, 429> = ArrayVec::new();
    memory.push(root);

    let mut current_directory: Address = Address { index: 0 };

    //construct the tree in memory
    for x in input {
        match x {
            CLI::cd(d) => match d {
                CLI_Directory::dir(name) => {
                    current_directory = memory
                        .get(current_directory.index)
                        .unwrap()
                        .children
                        .iter()
                        .map(|x| memory.get(x.index).unwrap())
                        .find(|c| c.name == *name)
                        .unwrap()
                        .index
                }
                CLI_Directory::dir_back => {
                    current_directory = memory.get(current_directory.index).unwrap().parent
                }
                CLI_Directory::root => current_directory = Address { index: 0 },
            },
            CLI::ls => {} //do nothing
            CLI::file(size, name) => {
                if already_found(&name.to_string(), current_directory, &memory) {
                    continue;
                }
                new_file(name.to_string(), *size, current_directory, &mut memory);
            }
            CLI::directory(d) => {
                let name = match d {
                    CLI_Directory::dir(d) => d.to_string(),
                    CLI_Directory::dir_back => panic!("file system contains loop"), //these could be solved
                    CLI_Directory::root => panic!("file system contains loop"), //these could be solved
                };
                if already_found(&name, current_directory, &memory) {
                    continue;
                }
                new_dir(name, current_directory, &mut memory);
            }
        }
    }

    let used_space: u32 = memory.get(0).unwrap().size;
    let target_to_delete: u32 = 30000000 - (70000000 - used_space);
    let size_of_folder_to_delete: u32 = memory
        .iter()
        .filter(|x| x.is_directory)
        .map(|x| x.size)
        .filter(|x| *x >= target_to_delete)
        .min()
        .unwrap();
    Output::U32(size_of_folder_to_delete)
}

fn already_found(name: &String, current_directory: Address, memory: &ArrayVec<File, 429>) -> bool {
    let found = memory
        .get(current_directory.index)
        .unwrap()
        .children
        .iter()
        .map(|x| memory.get(x.index).unwrap())
        .any(|x| x.name == *name);

    if found {
        println!("duplicate!")
    }
    return found;
}
