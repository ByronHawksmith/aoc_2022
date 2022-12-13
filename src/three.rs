use std::{collections::HashMap, env::current_dir, path::PathBuf};

// Create a function which matches each letter in the alphabet uppercase and lowercase to an integer
// value. The integer value should be the position of the letter in the alphabet. For example, A = 1,
// B = 2, C = 3, etc. The function should return a HashMap<String, i32> where the key is the letter
// and the value is the integer value.
fn get_alphabet_hashmap() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for (i, c) in alphabet.chars().enumerate() {
        map.insert(c.to_string(), (i + 1) as i32);
    }
    for (i, c) in alphabet.chars().enumerate() {
        map.insert(c.to_uppercase().to_string(), (i + 26 + 1) as i32);
    }
    map
}

// Create a function which takes a string parameter and splits it into two even substrings and returns a tuple
fn split_string(s: &str) -> (String, String) {
    let mut first = String::new();
    let mut second = String::new();
    let mut i = 0;
    for c in s.chars() {
        if i < s.len() / 2 {
            first.push(c);
        } else {
            second.push(c);
        }
        i += 1;
    }
    (first, second)
}

// Create a function which takes a (String, String) parameter and finds the SINGLE common letter in each string
// and returns the value
fn find_single_common_letter((first, second): (String, String)) -> String {
    let mut common = String::new();
    for c in first.chars() {
        if second.contains(c) {
            common.push(c);
            break;
        }
    }
    common
}

fn get_rucksacks_file() -> PathBuf {
    current_dir().unwrap().join("three.txt")
}

// Create a function that iterates over each line in a file, splits the line, finds the common letter in the two substrings and then gets the value of that common letter from the hashmap
fn get_priorities() -> Vec<i32> {
    let mut priorities = Vec::new();
    let file = get_rucksacks_file();
    let contents = std::fs::read_to_string(file).unwrap();
    let alphabet_hashmap = get_alphabet_hashmap();
    for line in contents.lines() {
        let (first, second) = split_string(line);
        let common = find_single_common_letter((first, second));
        let value = alphabet_hashmap.get(&common).unwrap();
        priorities.push(*value);
    }
    priorities
}

// Create a functiuon that sums the priorities
fn sum_priorities(priorities: Vec<i32>) -> i32 {
    let mut sum = 0;
    for priority in priorities {
        sum += priority;
    }
    sum
}

pub fn exec() {
    let priorities = get_priorities();
    let sum = sum_priorities(priorities);
    println!("Sum of priorities: {}", sum);
}
