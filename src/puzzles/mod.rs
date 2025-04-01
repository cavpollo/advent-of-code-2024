use std::fs::File;

pub mod puzzle01;
pub mod puzzle02;
pub mod puzzle03;
pub mod puzzle04;

pub trait Puzzle {
    fn run() -> i32;

    fn run_for_file(file: File) -> i32;
}
