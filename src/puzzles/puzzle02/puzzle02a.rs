#![allow(dead_code)]

use regex::Regex;
use crate::tools::file;

pub fn run() -> i32 {
    let mut safe_levels = 0;

    let regex = Regex::new(r"\s+").expect("Failed to parse the regex string");

    let buffer_lines = file::get_puzzle_buffer_lines(2);
    for read_line in buffer_lines {
        let line = read_line.expect("Failed to read the line from stdin");
        if line.trim().is_empty() {
            break;
        }
        let level_list: Vec<i32> = regex.split(&line)
            .map(|n| n.parse::<i32>().expect("Failed to parse line's first value to i32"))
            .collect();

        let is_safe = is_safe(&level_list);
        if is_safe {
            safe_levels += 1;
        }
    }

    safe_levels
}

fn is_safe(vector: &Vec<i32>) -> bool {
    if vector.is_empty() {
        return true;
    }

    // println!("vector: {:?}", vector);

    let mut last_value_difference: i32 = 0;
    let mut cursor_position: i32 = 0;

    loop {
        if cursor_position + 1 >= vector.len() as i32 {
            break;
        }

        let value_at_cursor = vector[cursor_position as usize];
        let next_value_at_cursor = vector[(cursor_position + 1) as usize];
        let value_difference = next_value_at_cursor - value_at_cursor;

        // println!("a: {:?}, b: {:?}, diff: {:?}", value_at_cursor, next_value_at_cursor, value_difference);

        if value_difference == 0 || value_difference.abs() > 3 {
            // println!("Not Safe difference!");
            return false;
        }

        if last_value_difference == 0 {
            last_value_difference = value_difference;
        } else if value_difference.is_positive() != last_value_difference.is_positive() {
            // println!("Not Safe direction!");
            return false
        }

        cursor_position += 1;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests_work() {
        let sample_vector_0 = vec![7, 6, 4, 2, 1];
        let sample_0_is_safe = is_safe(&sample_vector_0);
        assert_eq!(sample_0_is_safe, true);

        let sample_vector_1 = vec![1, 2, 7, 8, 9];
        let sample_1_is_safe = is_safe(&sample_vector_1);
        assert_eq!(sample_1_is_safe, false);

        let sample_vector_2 = vec![9, 7, 6, 2, 1];
        let sample_2_is_safe = is_safe(&sample_vector_2);
        assert_eq!(sample_2_is_safe, false);

        let sample_vector_3 = vec![1, 3, 2, 4, 5];
        let sample_3_is_safe = is_safe(&sample_vector_3);
        assert_eq!(sample_3_is_safe, false);

        let sample_vector_4 = vec![8, 6, 4, 4, 1];
        let sample_4_is_safe = is_safe(&sample_vector_4);
        assert_eq!(sample_4_is_safe, false);

        let sample_vector_5 = vec![1, 3, 6, 7, 9];
        let sample_5_is_safe = is_safe(&sample_vector_5);
        assert_eq!(sample_5_is_safe, true);
    }
}
