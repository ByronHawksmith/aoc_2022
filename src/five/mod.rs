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

// Declare a function that partitions a string that is 35 characters long into 9 lots of 4 characters
fn partition_string(s: &str) -> Vec<String> {
    let mut partitions = Vec::new();
    let mut start = 0;
    let mut end = 4;

    for _ in 0..9 {
        partitions.push(s[start..end].to_string());
        start += 4;
        end += if end + 4 > s.len() { 3 } else { 4 };
    }
    partitions
}

// Declare a function that uses the partition_string function to partition each line from the get_crates function
fn get_partitions() -> Vec<Vec<String>> {
    let (crates, _) = get_crates();
    let mut partitions = Vec::new();

    for crate_ in crates {
        partitions.push(partition_string(&crate_));
    }
    partitions
}

// Declare a function that reads from the crates file starting at a particular index and returns a vector of strings
fn get_moves(index: usize) -> Vec<String> {
    let file = get_crates_file();
    let contents = std::fs::read_to_string(file).unwrap();
    let mut moves = Vec::new();

    for line in contents.lines().skip(index + 1) {
        moves.push(line.to_string());
    }
    moves
}

// Declare a function that prints out the partitions
fn print_partitions() {
    let partitions = get_partitions();

    for partition in partitions {
        println!("{:?}", partition);
    }
}

pub fn exec() {
    print_partitions();
}
