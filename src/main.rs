mod execution_type;
mod puzzles;
#[macro_use]
mod tools;

use std::time::Instant;
use execution_type::ExecutionType;
use puzzles::puzzle02;

fn main() {
    execute_measuring_time!({
        puzzle02::puzzle02a::run(ExecutionType::Real);
    });
}
