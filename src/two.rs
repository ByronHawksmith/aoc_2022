use std::{collections::HashMap, env::current_dir, path::PathBuf};

#[derive(Copy, Clone, PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Default for Move {
    fn default() -> Self {
        Move::Rock
    }
}

#[derive(Copy, Clone)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

pub struct MatchResult {
    move_value: i32,
    outcome_value: i32,
}

pub fn get_move_value(m: Move) -> i32 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

pub fn get_outcome_value(o: Outcome) -> i32 {
    match o {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

pub fn get_strategy_map() -> HashMap<String, Move> {
    let mut map = HashMap::new();
    map.insert("A".to_string(), Move::Rock);
    map.insert("B".to_string(), Move::Paper);
    map.insert("C".to_string(), Move::Scissors);
    map.insert("X".to_string(), Move::Rock);
    map.insert("Y".to_string(), Move::Paper);
    map.insert("Z".to_string(), Move::Scissors);
    map
}

pub fn get_strategy_file() -> PathBuf {
    current_dir().unwrap().join("strategy.txt")
}

pub fn create_match_result(move_value: i32, outcome_value: i32) -> MatchResult {
    MatchResult {
        move_value,
        outcome_value,
    }
}

pub fn determine_winner(player1_move: Move, player2_move: Move) -> Outcome {
    if matches!(player1_move, Move::Rock) && matches!(player2_move, Move::Scissors) {
        Outcome::Win
    } else if matches!(player1_move, Move::Paper) && matches!(player2_move, Move::Rock) {
        Outcome::Win
    } else if matches!(player1_move, Move::Scissors) && matches!(player2_move, Move::Paper) {
        Outcome::Win
    } else if player1_move == player2_move {
        Outcome::Draw
    } else {
        Outcome::Lose
    }
}

pub fn determine_match_result(player1_move: Move, player2_move: Move) -> MatchResult {
    let player1_move_value = get_move_value(player1_move);
    let player1_outcome_value = get_outcome_value(determine_winner(player1_move, player2_move));

    create_match_result(player1_move_value, player1_outcome_value)
}

pub fn calculate_match_results() -> Vec<MatchResult> {
    let contents = std::fs::read_to_string(get_strategy_file())
        .expect("Something went wrong reading the file");
    let mut match_results = Vec::new();
    let strategy_map = get_strategy_map();

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        } else {
            let mut player1_move = Move::default();
            let mut player2_move = Move::default();

            for (i, c) in line.chars().filter(|c| !c.is_whitespace()).enumerate() {
                if i == 0 {
                    player2_move = strategy_map.get(&c.to_string()).unwrap().to_owned();
                } else {
                    player1_move = strategy_map.get(&c.to_string()).unwrap().to_owned();
                }
            }

            match_results.push(determine_match_result(player1_move, player2_move));
        }
    }

    match_results
}

pub fn calculate_total_score(match_results: Vec<MatchResult>) -> i32 {
    let mut total_score = 0;

    for match_result in match_results {
        total_score += match_result.move_value + match_result.outcome_value;
    }

    total_score
}

pub fn exec() {
    let match_results = calculate_match_results();
    println!("{}", calculate_total_score(match_results));
}
