#![allow(dead_code)]

use regex::Regex;
use crate::execution_type::ExecutionType;
use crate::tools::file;

pub fn run(execution_type: ExecutionType) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let regex = Regex::new(r"\s+").expect("Failed to parse the regex string");

    let buffer_lines = file::get_puzzle_buffer_lines(1, execution_type);
    for read_line in buffer_lines {
        let line = read_line.expect("Failed to read the line from stdin");
        if line.trim().is_empty() {
            break;
        }
        let parts: Vec<&str> = regex.split(&line).collect();

        let left_list_value = parts[0].parse::<i32>().expect("Failed to parse line's first value to i32");
        let position = get_insert_position(&left_list, left_list_value);
        left_list.insert(position, left_list_value);

        let right_list_value = parts[1].parse::<i32>().expect("Failed to parse line's second value to i32");
        let position = get_insert_position(&right_list, right_list_value);
        right_list.insert(position, right_list_value);
    }

    // println!("List1: {:?}", left_list);
    // println!("List2: {:?}", right_list);

    let mut total_difference = 0;
    for list_cursor in 0 .. right_list.len() {
        let difference = (left_list[list_cursor] - right_list[list_cursor]).abs();
        total_difference = total_difference + difference;
    }

    println!("Result: {}", total_difference);
}

fn get_insert_position(vector: &Vec<i32>, value_to_insert: i32) -> usize {
    if vector.is_empty() {
        return 0;
    }

    let mut lower_limit: i32 = 0;
    let mut higher_limit: i32 = vector.len() as i32 - 1;

    loop {
        let cursor_position = lower_limit + ((higher_limit - lower_limit) / 2);
        // println!("vector: {:?}, lower_limit: {}, higher_limit: {}, cursor_position: {}", vector, lower_limit, higher_limit, cursor_position);
        let value_at_cursor = vector[cursor_position as usize];

        if value_at_cursor == value_to_insert {
            return cursor_position as usize;
        } else if value_to_insert < value_at_cursor {
            higher_limit = cursor_position - 1;
        } else if value_to_insert > value_at_cursor {
            lower_limit = cursor_position + 1;
        }

        if higher_limit < lower_limit {
            return lower_limit as usize;
        }
    }
}
