use super::{
    contracts::Move,
    file::{get_crates_str, get_moves_str},
    utils::get_smallest_number,
};

fn partition_horizontal_slice(s: &str) -> Vec<String> {
    let mut partitions = Vec::new();
    let mut start = 0;
    let mut end = 4;

    for _ in 0..9 {
        partitions.push(s[start..end].to_string());
        start += 4;
        end = get_smallest_number(end + 4, s.len());
    }
    partitions
}

fn get_horizontal_crate_slices() -> Vec<Vec<String>> {
    let crates_str = get_crates_str();
    let mut horizontal_slices = Vec::new();

    for horizontal_slice in crates_str {
        horizontal_slices.push(partition_horizontal_slice(&horizontal_slice));
    }
    horizontal_slices
}

pub fn get_crate_stacks() -> Vec<Vec<String>> {
    let mut stacks = Vec::new();

    for i in 0..9 {
        let mut stack = Vec::new();
        for horizontal_slice in get_horizontal_crate_slices() {
            stack.push(horizontal_slice[i].trim().to_string());
        }
        stacks.push(stack);
    }
    stacks
}

pub fn get_moves() -> Vec<Move> {
    let moves = get_moves_str();
    let mut move_structs = Vec::new();

    for move_ in moves {
        let mut split = move_.split_whitespace();
        let number = split.nth(1).unwrap().parse::<u32>().unwrap();
        let source = split.nth(1).unwrap().parse::<u32>().unwrap();
        let destination = split.nth(1).unwrap().parse::<u32>().unwrap();
        move_structs.push(Move {
            number,
            source,
            destination,
        });
    }
    move_structs
}
