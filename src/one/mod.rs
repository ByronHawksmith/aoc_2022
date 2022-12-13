use std::{env::current_dir, fs::read_to_string, path::PathBuf};

fn get_calories_file() -> PathBuf {
    current_dir().unwrap().join("src/one/calories.txt")
}

fn calculate_calories() -> Vec<i32> {
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

fn get_largest_value(values: Vec<i32>) -> i32 {
    values.into_iter().max().unwrap_or(0)
}

pub fn exec() {
    println!("{}", get_largest_value(calculate_calories()));
}
