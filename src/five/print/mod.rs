use super::{parse_moves, transform::get_vertical_crate_stacks};

pub fn print_stacks() {
    let stacks = get_vertical_crate_stacks();

    for stack in stacks {
        println!("{:?}", stack);
    }
}

pub fn print_moves() {
    let move_structs = parse_moves();

    for move_ in move_structs {
        println!("{:?}", move_);
    }
}
