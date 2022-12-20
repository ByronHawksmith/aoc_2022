use super::{file::get_crates, utils::get_smallest_number};

fn partition_crates(s: &str) -> Vec<String> {
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
    let crates = get_crates();
    let mut partitions = Vec::new();

    for crate_ in crates {
        partitions.push(partition_crates(&crate_));
    }
    partitions
}

pub fn get_vertical_crate_stacks() -> Vec<Vec<String>> {
    let mut stacks = Vec::new();

    for i in 0..9 {
        let mut stack = Vec::new();
        for partition in get_horizontal_crate_slices() {
            stack.push(partition[i].to_string());
        }
        stacks.push(stack);
    }
    stacks
}
