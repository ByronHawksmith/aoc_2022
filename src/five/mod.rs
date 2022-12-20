use crate::five::print::{print_moves, print_stacks};

mod contracts;
mod file;
mod print;
mod transform;
mod utils;

pub fn exec() {
    print_stacks();
    println!("");
    print_moves();
}
