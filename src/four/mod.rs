use std::{env::current_dir, path::PathBuf};

fn get_assignment_pairs_file() -> PathBuf {
    current_dir().unwrap().join("src/four/assignment_pairs.txt")
}

struct Assignment {
    start: i32,
    end: i32,
}

struct AssignmentPair {
    first: Assignment,
    second: Assignment,
}

fn assignment_contains(a: &Assignment, b: &Assignment) -> bool {
    a.start <= b.start && a.end >= b.end
}

// Reads each line in the assignment_pairs.txt file and converts the line 18-23,15-22 into an AssignmentPair struct
fn get_assignment_pairs() -> Vec<AssignmentPair> {
    let file = get_assignment_pairs_file();
    let contents = std::fs::read_to_string(file).unwrap();
    let mut assignment_pairs = Vec::new();

    for line in contents.lines() {
        let mut split = line.split(',');
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        let mut first_split = first.split('-');
        let first_start = first_split.next().unwrap().parse::<i32>().unwrap();
        let first_end = first_split.next().unwrap().parse::<i32>().unwrap();
        let mut second_split = second.split('-');
        let second_start = second_split.next().unwrap().parse::<i32>().unwrap();
        let second_end = second_split.next().unwrap().parse::<i32>().unwrap();
        assignment_pairs.push(AssignmentPair {
            first: Assignment {
                start: first_start,
                end: first_end,
            },
            second: Assignment {
                start: second_start,
                end: second_end,
            },
        })
    }
    assignment_pairs
}

fn get_redundant_assignment_pairs() -> Vec<AssignmentPair> {
    let assignment_pairs = get_assignment_pairs();
    let mut redundant_assignments = Vec::new();

    for assignment_pair in assignment_pairs {
        if assignment_contains(&assignment_pair.first, &assignment_pair.second)
            || assignment_contains(&assignment_pair.second, &assignment_pair.first)
        {
            redundant_assignments.push(assignment_pair);
        }
    }
    redundant_assignments
}

pub fn exec() {
    let redundant_assignments = get_redundant_assignment_pairs();
    println!("{}", redundant_assignments.len());
}
