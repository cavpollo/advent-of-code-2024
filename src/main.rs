mod execution_type;
mod puzzles;
mod tools;

use execution_type::ExecutionType;
use puzzles::puzzle01;

fn main() {
    puzzle01::puzzle01a::run(ExecutionType::Sample);
    puzzle01::puzzle01a::run(ExecutionType::Real);
}
