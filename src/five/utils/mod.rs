pub fn get_smallest_number(left: usize, right: usize) -> usize {
    if left > right {
        right
    } else {
        left
    }
}
