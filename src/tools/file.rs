use std::fs::File;
use std::path::{Path, PathBuf};
use std::env;

pub fn get_puzzle_input_file(
    puzzle_id: i8,
) -> File {
    let input_file_name = get_input_file_name(puzzle_id);
    let input_file_path = get_file_path(input_file_name);

    // println!("input_file_path: {:?}", input_file_path);

    get_file(&input_file_path)
}

#[allow(dead_code)]
pub fn get_puzzle_sample_file(
    puzzle_id: i8, sample_id: i8,
) -> File {
    let sample_file_name = get_sample_file_name(puzzle_id, sample_id);
    let sample_file_path = get_file_path(sample_file_name);

    // println!("sample_file_path: {:?}", sample_file_path);

    get_file(&sample_file_path)
}

fn get_input_file_name(
    puzzle_id: i8,
) -> String {
    let zero_padded_puzzle_id = format!("{:02}", puzzle_id);
    format!("assets/puzzle{}_input.txt", zero_padded_puzzle_id)
}

#[allow(dead_code)]
fn get_sample_file_name(
    puzzle_id: i8,
    sample_id: i8,
) -> String {
    let zero_padded_puzzle_id = format!("{:02}", puzzle_id);
    let zero_padded_sample_id = format!("{:02}", sample_id);
    format!("assets/puzzle{}_sample_{}.txt", zero_padded_puzzle_id, zero_padded_sample_id)
}

fn get_file_path(
    file_name: String,
) -> PathBuf {
    let current_path = env::current_dir().expect("Failed to get current directory");
    let mut file_path = current_path.clone();
    file_path.push(file_name);
    file_path
}

fn get_file(file_path: &PathBuf) -> File {
    let path = Path::new(&file_path);
    File::open(&path).expect("Failed to open file")
}
