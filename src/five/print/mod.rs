use super::transform::{get_crate_stacks, get_moves};

pub fn print_stacks() {
    let stacks = get_crate_stacks();

    for stack in stacks {
        println!("{:?}", stack);
    }
}

pub fn print_stacks_p(stacks: Vec<Vec<String>>) {
    for stack in stacks {
        println!("{:?}", stack);
    }
}

pub fn print_moves() {
    let move_structs = get_moves();

    for move_ in move_structs {
        println!("{:?}", move_);
    }
}

pub fn print_top_crates_p(top_crates: Vec<String>) {
    println!("{:?}", top_crates);
}
