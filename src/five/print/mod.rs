use super::transform::{get_crate_stacks, get_moves};

pub fn print_stacks() {
    let stacks = get_crate_stacks();

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
