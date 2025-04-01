use crate::puzzles::Puzzle;
use crate::tools::file;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Puzzle04a;

impl Puzzle04a {

}

impl Puzzle for Puzzle04a {
    fn run() -> i32 {
        let puzzle_file = file::get_puzzle_input_file(4);
        Puzzle04a::run_for_file(puzzle_file)
    }

    fn run_for_file(file: File) -> i32 {
        let xmas = "XMAS";
        let xmas_reversed = xmas.chars().rev().collect::<String>().clone();

        // X position, matching direction, String position, tracked String
        let mut last_trackers: Vec<(i32, i8, i8, &str)> = Vec::new();
        let mut current_trackers: Vec<(i32, i8, i8, &str)> = Vec::new();
        let mut cumulative_result= 0;

        for read_line in BufReader::new(file).lines() {
            let line = read_line.expect("Failed to read the line from stdin");
            if line.trim().is_empty() {
                break;
            }

            current_trackers = last_trackers.clone();
            last_trackers = Vec::new();

            // println!();
            // println!("********");
            // println!("line: {}", line);
            // println!("********");

            let mut current_tracked_pair: Option<(i32, &str)> = None;
            for (index, char) in line.char_indices() {

                // println!("==> char: {}", char);

                current_trackers.iter()
                    .filter(|tracker| { tracker.0 == index as i32 && tracker.3.chars().nth(tracker.2 as usize).unwrap() == char})
                    .for_each(|tracker| {
                        let direction = tracker.1;
                        // print!("tracker - x pos: {}, direction: {}, word pos: {}, word: {}", tracker.0, direction, tracker.2, tracker.3);

                        let can_continue = (index as i32 + direction as i32) >= 0 &&
                            (index as i32 + direction as i32) < line.len() as i32;
                        if tracker.2 as i32 + 1 == tracker.3.len() as i32 {
                            cumulative_result += 1;
                            // print!(" - full match {}", cumulative_result);

                            if can_continue {
                                // println!(" - can continue");
                                if tracker.3 == xmas {
                                    last_trackers.push((index as i32 + direction as i32, direction, 1, &xmas_reversed as &str))
                                } else {
                                    last_trackers.push((index as i32 + direction as i32, direction, 1, &xmas as &str))
                                }
                            } else {
                                // println!()
                            }
                        } else {
                            if can_continue {
                                // println!(" - can continue");
                                last_trackers.push((index as i32 + direction as i32, direction, tracker.2 + 1, tracker.3))
                            } else {
                                // println!()
                            }
                        }
                    });

                if current_tracked_pair != None {
                    let mut_tracked_pair = current_tracked_pair.as_mut().unwrap();
                    let tracked_position: &mut i32 = &mut mut_tracked_pair.0;
                    let tracked_word = mut_tracked_pair.1;
                    // print!("pos: {}, word: {}", tracked_position, tracked_word);

                    if char == tracked_word.chars().nth(*tracked_position as usize).unwrap() {
                        *tracked_position += 1;
                        // print!(" - continue matching: {}", *tracked_position);

                        if *tracked_position == tracked_word.len() as i32 {
                            cumulative_result += 1;
                            // println!(" - full match {}", cumulative_result);
                            current_tracked_pair = None;
                        } else {
                            // println!()
                        }
                    } else {
                        // println!(" - can't continue matching");
                        current_tracked_pair = None;
                    }
                }

                if current_tracked_pair == None {
                    if char == xmas.chars().nth(0).unwrap() {
                        // println!("XMAS first match");
                        current_tracked_pair = Some((1, &xmas));

                        for direction in -1..=1 {
                            let new_tracked_position: (i32, i8, i8, &str) = (index as i32 + direction as i32, direction, 1, &xmas);
                            if !last_trackers.contains(&new_tracked_position) {
                                last_trackers.push(new_tracked_position);
                            }
                        }
                    } else if char == xmas_reversed.chars().nth(0).unwrap() {
                        // println!("SAMX first match");
                        current_tracked_pair = Some((1, &xmas_reversed));

                        for direction in -1..=1 {
                            let new_tracked_position: (i32, i8, i8, &str) = (index as i32 + direction as i32, direction, 1, &xmas_reversed);
                            if !last_trackers.contains(&new_tracked_position) {
                                last_trackers.push(new_tracked_position);
                            }
                        }
                    } else {
                        // println!("no match");
                    }
                }
            }
        }

        cumulative_result
    }
}

// Direction:
// _ _ _ _
// _ X _ _
//-1 0 1 _
// _ _ _ _

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests_work() {
        let puzzle_file = file::get_puzzle_sample_file(4, 1);
        let result = Puzzle04a::run_for_file(puzzle_file);
        assert_eq!(result, 4);

        let puzzle_file = file::get_puzzle_sample_file(4, 2);
        let result = Puzzle04a::run_for_file(puzzle_file);
        assert_eq!(result, 18);

        let puzzle_file = file::get_puzzle_sample_file(4, 3);
        let result = Puzzle04a::run_for_file(puzzle_file);
        assert_eq!(result, 8);

        let puzzle_file = file::get_puzzle_sample_file(4, 4);
        let result = Puzzle04a::run_for_file(puzzle_file);
        assert_eq!(result, 8);

        let puzzle_file = file::get_puzzle_sample_file(4, 5);
        let result = Puzzle04a::run_for_file(puzzle_file);
        assert_eq!(result, 10);
    }
}
