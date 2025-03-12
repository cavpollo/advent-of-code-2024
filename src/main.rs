mod execution_type;
mod puzzles;
#[macro_use]
mod tools;

use std::time::Instant;
use execution_type::ExecutionType;
use puzzles::puzzle01;

fn main() {
    execute_measuring_time!({
        // puzzle01::puzzle01a::run(ExecutionType::Real);
        // puzzle01::puzzle01b::run(ExecutionType::Sample);
        puzzle01::puzzle01b::run(ExecutionType::Real);
    });
}
