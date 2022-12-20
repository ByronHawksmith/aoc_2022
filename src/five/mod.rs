use crate::five::print::{print_stacks, print_top_crates_p};

use self::utils::{find_first_empty_slot, find_first_populated_slot};

mod contracts;
mod file;
mod print;
mod transform;
mod utils;

fn pop_crates(input: &mut Vec<String>, n: usize) -> Vec<String> {
    let mut output = Vec::new();
    for _ in 0..n {
        let index = find_first_populated_slot(input.to_vec());
        match index {
            Some(i) => {
                output.push(input[i].clone());
                input[i] = "".to_string();
            }
            None => panic!("No crates to pop!"),
        }
    }
    output
}

fn push_crates(destination: &mut Vec<String>, crates: Vec<String>) {
    for c in crates {
        let index = find_first_empty_slot(destination.to_vec());
        match index {
            Some(index) => destination[index] = c,
            None => destination.insert(0, c),
        }
    }
}

pub fn apply_moves() -> Vec<Vec<String>> {
    let moves = transform::get_moves();
    let mut stacks = transform::get_crate_stacks();

    for move_ in moves {
        let source_index = (move_.source - 1) as usize;
        let destination_index = (move_.destination - 1) as usize;
        let mut source_stack = stacks[source_index].clone();
        let mut destination_stack = stacks[destination_index].clone();
        let number = move_.number as usize;

        let crates_to_transfer = pop_crates(&mut source_stack, number);
        push_crates(&mut destination_stack, crates_to_transfer);

        stacks[source_index] = source_stack;
        stacks[destination_index] = destination_stack;
    }
    stacks
}

fn get_top_crate_in_each_stack(stack: Vec<Vec<String>>) -> Vec<String> {
    let mut top: Vec<String> = Vec::new();
    for i in 0..stack.len() {
        let first_populated_slot = find_first_populated_slot(stack[i].to_vec());
        match first_populated_slot {
            Some(index) => top.push(stack[i][index].clone()),
            None => top.push(String::from("")),
        }
    }
    top
}

pub fn exec() {
    print_stacks();
    println!("");
    let stacks = apply_moves();
    let top_crates = get_top_crate_in_each_stack(stacks);
    print_top_crates_p(top_crates);
}
