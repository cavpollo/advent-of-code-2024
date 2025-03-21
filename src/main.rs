mod puzzles;
#[macro_use]
mod tools;

use puzzles::puzzle03::puzzle03b::Puzzle03b;
use puzzles::Puzzle;

fn main() {
    let result = execute_measuring_time!(Puzzle03b::run);
    println!("Result: {}", result);
}
