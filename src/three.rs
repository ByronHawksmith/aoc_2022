use std::{collections::HashMap, env::current_dir, fs, path::PathBuf};

fn get_alphabet_hashmap() -> HashMap<char, i32> {
    let mut map = HashMap::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for (i, c) in alphabet.chars().enumerate() {
        map.insert(c, (i + 1) as i32);
    }
    for (i, c) in alphabet.chars().enumerate() {
        map.insert(c.to_uppercase().next().unwrap(), (i + 26 + 1) as i32);
    }
    map
}

fn split_string(s: &str) -> (String, String) {
    let mid = s.len() / 2;
    let first = s[..mid].to_string();
    let second = s[mid..].to_string();
    (first, second)
}

fn find_single_common_letter((first, second): (String, String)) -> char {
    for c in first.chars() {
        if second.contains(c) {
            return c;
        }
    }
    ' '
}

fn get_rucksacks_file() -> PathBuf {
    current_dir().unwrap().join("three.txt")
}

fn get_priorities() -> Vec<i32> {
    let file = get_rucksacks_file();
    let contents = fs::read_to_string(file).unwrap();
    let alphabet_hashmap = get_alphabet_hashmap();
    let mut priorities = Vec::new();

    for line in contents.lines() {
        let (first, second) = split_string(line);
        let common = find_single_common_letter((first, second));
        let value = *alphabet_hashmap.get(&common).unwrap_or(&0);
        priorities.push(value);
    }
    priorities
}

fn sum_priorities(priorities: Vec<i32>) -> i32 {
    priorities.iter().sum()
}

pub fn exec() {
    let priorities = get_priorities();
    let sum = sum_priorities(priorities);
    println!("Sum of priorities: {}", sum);
}
