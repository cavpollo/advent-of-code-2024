mod puzzles;
#[macro_use]
mod tools;

use std::env;
use puzzles::puzzle01::puzzle01a::Puzzle01a;
use puzzles::puzzle01::puzzle01b::Puzzle01b;
use puzzles::puzzle02::puzzle02a::Puzzle02a;
use puzzles::puzzle02::puzzle02b::Puzzle02b;
use puzzles::puzzle03::puzzle03a::Puzzle03a;
use puzzles::puzzle03::puzzle03b::Puzzle03b;
use puzzles::puzzle04::puzzle04a::Puzzle04a;
use puzzles::Puzzle;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    if args.len() < 2 {
        panic!("Must provide the puzzle id as the second argument.");
    }
    let puzzle_id = &args[1];

    println!("Executing Puzzle {}", puzzle_id);
    let puzzle = get_puzzle(puzzle_id);
    let result = execute_measuring_time!(puzzle);
    println!("Result: {}", result);
}

fn get_puzzle(puzzle_id: &str) -> fn() -> i32 {
     match puzzle_id {
        "1a" => Puzzle01a::run,
        "1b" => Puzzle01b::run,
        "2a" => Puzzle02a::run,
        "2b" => Puzzle02b::run,
        "3a" => Puzzle03a::run,
        "3b" => Puzzle03b::run,
        "4a" => Puzzle04a::run,
        _ => panic!("Unknown level id {}.", puzzle_id),
    }
}
