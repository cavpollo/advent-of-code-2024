use crate::puzzles::Puzzle;
use crate::tools::file;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Puzzle01b;

impl Puzzle01b {
    fn insert_value(vector: &mut Vec<(i32, i16)>, value_to_insert: i32) {
        if vector.is_empty() {
            let pair = (value_to_insert, 1i16);
            vector.insert(0, pair);
            return;
        }

        let mut lower_limit: i32 = 0;
        let mut higher_limit: i32 = vector.len() as i32 - 1;

        loop {
            let cursor_position = lower_limit + ((higher_limit - lower_limit) / 2);
            // println!("vector: {:?}, lower_limit: {}, higher_limit: {}, cursor_position: {}", vector, lower_limit, higher_limit, cursor_position);
            let (value_at_cursor, occurrences_at_cursor) = vector.get_mut(cursor_position as usize).expect("Failed to get mutable pair at cursor position");

            if *value_at_cursor == value_to_insert {
                *occurrences_at_cursor += 1;
                break;
            } else if value_to_insert < *value_at_cursor {
                higher_limit = cursor_position - 1;
            } else if value_to_insert > *value_at_cursor {
                lower_limit = cursor_position + 1;
            }

            if higher_limit < lower_limit {
                let pair = (value_to_insert, 1i16);
                vector.insert(lower_limit as usize, pair);
                break;
            }
        }
    }

    fn get_value_count(vector: &Vec<(i32, i16)>, value: i32) -> Option<i16> {
        if vector.is_empty() {
            return None;
        }

        let mut lower_limit: i32 = 0;
        let mut higher_limit: i32 = vector.len() as i32 - 1;

        loop {
            let cursor_position = lower_limit + ((higher_limit - lower_limit) / 2);
            // println!("vector: {:?}, lower_limit: {}, higher_limit: {}, cursor_position: {}", vector, lower_limit, higher_limit, cursor_position);
            let (value_at_cursor, occurrences_at_cursor) = vector[cursor_position as usize];

            if value_at_cursor == value {
                return Some(occurrences_at_cursor);
            } else if value < value_at_cursor {
                higher_limit = cursor_position - 1;
            } else if value > value_at_cursor {
                lower_limit = cursor_position + 1;
            }

            if higher_limit < lower_limit {
                return None;
            }
        }
    }
}

impl Puzzle for Puzzle01b {
    fn run() -> i32 {
        let puzzle_file = file::get_puzzle_input_file(1);
        Puzzle01b::run_for_file(puzzle_file)
    }

    fn run_for_file(file: File) -> i32 {
        let mut left_list: Vec<(i32, i16)> = Vec::new();
        let mut right_list: Vec<(i32, i16)> = Vec::new();

        let regex = Regex::new(r"\s+").expect("Failed to parse the regex string");

        for read_line in BufReader::new(file).lines() {
            let line = read_line.expect("Failed to read the line from stdin");
            if line.trim().is_empty() {
                break;
            }
            let parts: Vec<&str> = regex.split(&line).collect();

            let left_list_value = parts[0].parse::<i32>().expect("Failed to parse line's first value to i32");
            Puzzle01b::insert_value(&mut left_list, left_list_value);

            let right_list_value = parts[1].parse::<i32>().expect("Failed to parse line's second value to i32");
            Puzzle01b::insert_value(&mut right_list, right_list_value);
        }

        // println!("List1: {:?}", left_list);
        // println!("List2: {:?}", right_list);

        let mut total_difference = 0;
        for list_cursor in 0..left_list.len() {
            let (value, occurrences) = left_list[list_cursor];
            let count = Puzzle01b::get_value_count(&right_list, value);

            match count {
                Some(count) => {
                    // println!("({} * {}) * {}", value, count, occurrences);
                    total_difference += (value * count as i32) * occurrences as i32
                },
                None => continue,
            }
        }

        total_difference
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests_work() {
        let mut sample_vector: Vec<(i32, i16)> = vec![];
        Puzzle01b::insert_value(&mut sample_vector, 3);
        assert_eq!(sample_vector[0].0, 3);
        assert_eq!(sample_vector[0].1, 1);

        Puzzle01b::insert_value(&mut sample_vector, 4);
        assert_eq!(sample_vector[0].0, 3);
        assert_eq!(sample_vector[0].1, 1);
        assert_eq!(sample_vector[1].0, 4);
        assert_eq!(sample_vector[1].1, 1);

        Puzzle01b::insert_value(&mut sample_vector, 2);
        assert_eq!(sample_vector[0].0, 2);
        assert_eq!(sample_vector[0].1, 1);
        assert_eq!(sample_vector[1].0, 3);
        assert_eq!(sample_vector[1].1, 1);
        assert_eq!(sample_vector[2].0, 4);
        assert_eq!(sample_vector[2].1, 1);

        Puzzle01b::insert_value(&mut sample_vector, 1);
        assert_eq!(sample_vector[0].0, 1);
        assert_eq!(sample_vector[0].1, 1);
        assert_eq!(sample_vector[1].0, 2);
        assert_eq!(sample_vector[1].1, 1);
        assert_eq!(sample_vector[2].0, 3);
        assert_eq!(sample_vector[2].1, 1);
        assert_eq!(sample_vector[3].0, 4);
        assert_eq!(sample_vector[3].1, 1);

        Puzzle01b::insert_value(&mut sample_vector, 3);
        assert_eq!(sample_vector[0].0, 1);
        assert_eq!(sample_vector[0].1, 1);
        assert_eq!(sample_vector[1].0, 2);
        assert_eq!(sample_vector[1].1, 1);
        assert_eq!(sample_vector[2].0, 3);
        assert_eq!(sample_vector[2].1, 2);
        assert_eq!(sample_vector[3].0, 4);
        assert_eq!(sample_vector[3].1, 1);

        Puzzle01b::insert_value(&mut sample_vector, 3);
        assert_eq!(sample_vector[0].0, 1);
        assert_eq!(sample_vector[0].1, 1);
        assert_eq!(sample_vector[1].0, 2);
        assert_eq!(sample_vector[1].1, 1);
        assert_eq!(sample_vector[2].0, 3);
        assert_eq!(sample_vector[2].1, 3);
        assert_eq!(sample_vector[3].0, 4);
        assert_eq!(sample_vector[3].1, 1);
    }
}
