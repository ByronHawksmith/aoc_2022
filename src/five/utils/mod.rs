pub fn get_smallest_number(left: usize, right: usize) -> usize {
    if left > right {
        right
    } else {
        left
    }
}

pub fn find_first_populated_slot(input: Vec<String>) -> Option<usize> {
    for (i, s) in input.iter().enumerate() {
        if s != "" {
            return Some(i);
        }
    }
    None
}

pub fn find_first_empty_slot(input: Vec<String>) -> Option<usize> {
    for (i, s) in input.iter().rev().enumerate() {
        if s == "" {
            return Some(input.len() - i - 1);
        }
    }
    None
}
