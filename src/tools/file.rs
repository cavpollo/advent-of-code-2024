use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::env;
use crate::execution_type::ExecutionType;

pub fn get_puzzle_buffer_lines(
    puzzle_id: i8,
    execution_type: ExecutionType,
) -> Lines<BufReader<File>> {
    let file_path = get_file_path(puzzle_id, execution_type);

    // println!("file_path: {:?}", file_path);

    get_file_buffer_lines(&file_path)
}

fn get_file_path(
    puzzle_id: i8,
    execution_type: ExecutionType,
) -> PathBuf {
    let current_path = env::current_dir().expect("Failed to get current directory");
    let file_name = get_file_name(puzzle_id, execution_type);
    let mut file_path = current_path.clone();
    file_path.push(file_name);
    file_path
}

fn get_file_name(
    puzzle_id: i8,
    execution_type: ExecutionType,
) -> String {
    let zero_padded_puzzle_id = format!("{:02}", puzzle_id);
    match execution_type {
        ExecutionType::Sample => format!("assets/puzzle{}_sample_input.txt", zero_padded_puzzle_id),
        ExecutionType::Real => format!("assets/puzzle{}_real_input.txt", zero_padded_puzzle_id),
    }
}

fn get_file_buffer_lines(file_path: &PathBuf) -> Lines<BufReader<File>> {
    let path = Path::new(&file_path);
    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader.lines()
}
