use crate::puzzles::Puzzle;
use crate::tools::file;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Puzzle04b;

impl Puzzle04b {

}

impl Puzzle for Puzzle04b {
    fn run() -> i32 {
        let puzzle_file = file::get_puzzle_input_file(4);
        Puzzle04b::run_for_file(puzzle_file)
    }

    fn run_for_file(file: File) -> i32 {
        // piece:
        // -2 = S.S
        // -1 = M.S
        // +1 = S.M
        // +2 = M.M
        // X center position, piece, row
        let mut prev_row_trackers: Vec<(i32, i8, i8)> = Vec::new();
        let mut current_row_trackers: Vec<(i32, i8, i8)> = Vec::new();
        let mut cumulative_result= 0;

        for read_line in BufReader::new(file).lines() {
            let line = read_line.expect("Failed to read the line from stdin");
            if line.trim().is_empty() {
                break;
            }

            // println!();
            // println!("********");
            // println!("line: {}", line);
            // println!("prev_row_trackers {:?}", prev_row_trackers);
            // println!("********");

            let mut first_char = '.';
            let mut second_char = '.';

            // let mut current_tracked_pair: Option<(i32, &str)> = None;
            for (index, current_char) in line.char_indices() {
                // print!("==> chars: {}{}{}, index {}", first_char, second_char, current_char, index);

                if current_char == 'A' {
                    // print!(" - match A: {:?}", prev_row_trackers);
                    let prev_line = prev_row_trackers.iter()
                        .find(|row| row.0 == index as i32 && row.2 == 0);
                    // print!(": {:?}", prev_line);
                    if prev_line.is_some() {
                        // print!(" - success {:?}", prev_line.unwrap());
                        current_row_trackers.push((index as i32, prev_line.unwrap().1, 1));
                    }
                } else if index >= 2 && current_char == 'S' {
                    // print!(" - match S");
                    if first_char == 'S' {
                        // print!(", S");
                        let prev_line = prev_row_trackers.iter()
                            .find(|row| row.0 + 1 == index as i32 && row.2 == 1);
                        // print!(": {:?}", prev_line);
                        if prev_line.is_some() && prev_line.unwrap().1 == 2 {
                            // print!(" - success {:?}", prev_line.unwrap());
                            cumulative_result += 1;
                        }

                        current_row_trackers.push((index as i32 -1, -2, 0));
                    } else if first_char == 'M' {
                        // print!(", M");
                        let prev_line = prev_row_trackers.iter()
                            .find(|row| row.0 + 1 == index as i32 && row.2 == 1);
                        // print!(": {:?}", prev_line);
                        if prev_line.is_some() && prev_line.unwrap().1 == -1 {
                            // print!(" - success {:?}", prev_line.unwrap());
                            cumulative_result += 1;
                        }

                        current_row_trackers.push((index as i32 -1, -1, 0));
                    }
                } else if index >= 2 && current_char == 'M' {
                    // print!(" - match M");
                    if first_char == 'S' {
                        // print!(", S");
                        let prev_line = prev_row_trackers.iter()
                            .find(|row| row.0 + 1 == index as i32 && row.2 == 1);
                        // print!(": {:?}", prev_line);
                        if prev_line.is_some() && prev_line.unwrap().1 == 1 {
                            // print!(" - success {:?}", prev_line.unwrap());
                            cumulative_result += 1;
                        }

                        current_row_trackers.push((index as i32 -1, 1, 0));
                    } else if first_char == 'M' {
                        // print!(", M");
                        let prev_line = prev_row_trackers.iter()
                            .find(|row| row.0 + 1 == index as i32 && row.2 == 1);
                        // print!(": {:?}", prev_line);
                        if prev_line.is_some() && prev_line.unwrap().1 == -2 {
                            // print!(" - success {:?}", prev_line.unwrap());
                            cumulative_result += 1;
                        }

                        current_row_trackers.push((index as i32 -1, 2, 0));
                    }
                }
                // println!();

                first_char = second_char;
                second_char = current_char;
            }

            prev_row_trackers = current_row_trackers.clone();
            current_row_trackers = Vec::new();
        }

        cumulative_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests_work() {
        let puzzle_file = file::get_puzzle_sample_file(4, 6);
        let result = Puzzle04b::run_for_file(puzzle_file);
        assert_eq!(result, 1);

        let puzzle_file = file::get_puzzle_sample_file(4, 7);
        let result = Puzzle04b::run_for_file(puzzle_file);
        assert_eq!(result, 4);

        let puzzle_file = file::get_puzzle_sample_file(4, 8);
        let result = Puzzle04b::run_for_file(puzzle_file);
        assert_eq!(result, 9);
    }
}
