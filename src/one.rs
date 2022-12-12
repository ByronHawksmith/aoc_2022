
use std::{env::current_dir, fs::read_to_string, path::PathBuf};

pub fn get_calories_file() -> PathBuf {
    current_dir().unwrap().join("calories.txt")
}

pub fn calculate_calories() -> Vec<i32> {
    let contents =
        read_to_string(get_calories_file()).expect("Something went wrong reading the file");
    let mut sum = 0;
    let mut sums = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    sums.push(sum);
    sums
}

pub fn get_largest_value(values: Vec<i32>) -> i32 {
    values.into_iter().max().unwrap_or(0)
}
