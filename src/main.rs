mod puzzles;
#[macro_use]
mod tools;

use std::time::Instant;
use puzzles::puzzle02;

fn main() {
    execute_measuring_time!({
        puzzle02::puzzle02b::run();
    });
}
