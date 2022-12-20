use std::{env::current_dir, path::PathBuf};

fn get_crates_file() -> PathBuf {
    current_dir().unwrap().join("src/five/crates.txt")
}

pub fn get_moves_index() -> usize {
    let crates_file = get_crates_file();
    let crates = std::fs::read_to_string(crates_file).unwrap();
    let mut index = 0;

    for (i, line) in crates.lines().enumerate() {
        if line.is_empty() {
            index = i;
            break;
        }
    }
    index
}

pub fn get_crates_str() -> Vec<String> {
    let file = get_crates_file();
    let contents = std::fs::read_to_string(file).unwrap();
    let mut crates = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            break;
        }
        crates.push(line.to_string());
    }
    crates
}

pub fn get_moves_str() -> Vec<String> {
    let file = get_crates_file();
    let contents = std::fs::read_to_string(file).unwrap();
    let mut moves = Vec::new();

    for line in contents.lines().skip(get_moves_index() + 1) {
        moves.push(line.to_string());
    }
    moves
}
