mod puzzles;
#[macro_use]
mod tools;

use puzzles::puzzle03::puzzle03a::Puzzle03a;
use puzzles::Puzzle;

fn main() {
    let result = execute_measuring_time!(Puzzle03a::run);
    println!("Result: {}", result);
}
