mod contracts;
mod file;
mod print;
mod transform;
mod utils;

use self::{
    contracts::Move,
    file::get_moves,
    print::{print_moves, print_stacks},
};

// Call get_moves and parse each line into a Move struct, the structure of a move line is: move 1 from 2 to 1
fn parse_moves() -> Vec<Move> {
    let moves = get_moves();
    let mut move_structs = Vec::new();

    for move_ in moves {
        let mut split = move_.split_whitespace();
        let number = split.nth(1).unwrap().parse::<u32>().unwrap();
        let source = split.nth(1).unwrap().parse::<u32>().unwrap();
        let destination = split.nth(1).unwrap().parse::<u32>().unwrap();
        move_structs.push(Move {
            number,
            source,
            destination,
        });
    }
    move_structs
}

pub fn exec() {
    print_stacks();
    println!("");
    print_moves();
}
