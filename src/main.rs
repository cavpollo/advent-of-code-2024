mod puzzles;
#[macro_use]
mod tools;

use puzzles::puzzle02::puzzle02b::Puzzle02b;
use puzzles::Puzzle;

fn main() {
    let result = execute_measuring_time!(Puzzle02b::run);
    println!("Result: {}", result);
}
