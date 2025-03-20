mod puzzles;
#[macro_use]
mod tools;

use puzzles::puzzle02;

fn main() {
    let result = execute_measuring_time!(puzzle02::puzzle02b::run);
    println!("Result: {}", result);
}
