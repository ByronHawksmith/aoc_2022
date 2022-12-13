mod utils;

use std::{env::current_dir, path::PathBuf};

fn get_crates_file() -> PathBuf {
    current_dir().unwrap().join("src/five/crates.txt")
}

fn get_crates() -> (Vec<String>, usize) {
    let file = get_crates_file();
    let contents = std::fs::read_to_string(file).unwrap();
    let mut crates = Vec::new();
    let mut index = 0;

    for (i, line) in contents.lines().enumerate() {
        if line.is_empty() {
            index = i;
            break;
        }
        crates.push(line.to_string());
    }
    (crates, index)
}

fn partition_crates(s: &str) -> Vec<String> {
    let mut partitions = Vec::new();
    let mut start = 0;
    let mut end = 4;

    for _ in 0..9 {
        partitions.push(s[start..end].to_string());
        start += 4;
        end = utils::get_smallest_number(end + 4, s.len());
    }
    partitions
}

fn get_crate_partitions() -> Vec<Vec<String>> {
    let (crates, _) = get_crates();
    let mut partitions = Vec::new();

    for crate_ in crates {
        partitions.push(partition_crates(&crate_));
    }
    partitions
}

fn get_crate_stacks(crate_partitions: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut stacks = Vec::new();

    for i in 0..9 {
        let mut stack = Vec::new();
        for partition in crate_partitions {
            stack.push(partition[i].to_string());
        }
        stacks.push(stack);
    }

    stacks
}

fn get_moves(index: usize) -> Vec<String> {
    let file = get_crates_file();
    let contents = std::fs::read_to_string(file).unwrap();
    let mut moves = Vec::new();

    for line in contents.lines().skip(index + 1) {
        moves.push(line.to_string());
    }
    moves
}

fn print_partitions() {
    let partitions = get_crate_partitions();

    for partition in partitions {
        println!("{:?}", partition);
    }
}

fn print_stacks() {
    let stacks = get_crate_stacks(&get_crate_partitions());

    for stack in stacks {
        println!("{:?}", stack);
    }
}

pub fn exec() {
    print_partitions();
    print_stacks();
}
