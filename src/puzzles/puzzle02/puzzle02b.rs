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
        // println!("SAFE: {}\n", is_safe);
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
    let mut failures: i32 = 0;
    let mut offset_0: i32 = 0;
    let mut offset_1: i32 = 0;

    loop {
        if failures > 1 {
            // println!("TOO MANY FAILURES");
            return false;
        }
        if cursor_position + 1 + offset_1 >= vector.len() as i32 {
            break;
        }

        let value_at_0 = vector[(cursor_position + offset_0) as usize];
        let value_at_1 = vector[(cursor_position + 1 + offset_1) as usize];
        // print!("a: {:?}, b: {:?}, offset0: {:?}, offset1: {:?} ", value_at_0, value_at_1, offset_0, offset_1);

        let value_0_1_difference = value_at_1 - value_at_0;
        if value_0_1_difference != 0 && value_0_1_difference.abs() <= 3 {

            if last_value_difference == 0 {
                // No known differences yet. Let's assume it is correct and move on to the next one.
                last_value_difference = value_0_1_difference;
                cursor_position += 1 + offset_1;
                offset_0 = 0;
                offset_1 = 0;

                // println!("FIRST VALUE");
                continue;
            } else if value_0_1_difference.is_positive() == last_value_difference.is_positive() {
                // Same direction, all good.
                cursor_position += 1 + offset_1;
                offset_0 = 0;
                offset_1 = 0;

                // println!("OK SAME DIRECTION");
                continue;
            }
        }

        // Difference is invalid

        // First failure, let's try skipping the right value
        if offset_1 == 0 {
            failures += 1;
            offset_1 = 1;
            // println!("FAILURE - SKIP RIGHT VALUE");
        } else {
            // Second failure

            // We are at the start. Let's try skipping the left value
            if cursor_position == 0 {
                // We already tried this
                if offset_0 != 0 {
                    // Can't fix
                    // println!("CAN'T FIX");
                    return false;
                }

                offset_0 = 1;
                offset_1 = 1;
                // println!("FAILURE - SKIP LEFT VALUE AT START");
            } else  {
                last_value_difference = 0;
                cursor_position -= 1;
                // println!("FAILURE - RESET AND GO BACK");
            }
        }
    }

    true
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
        assert_eq!(sample_3_is_safe, true);

        let sample_vector_4 = vec![8, 6, 4, 4, 1];
        let sample_4_is_safe = is_safe(&sample_vector_4);
        assert_eq!(sample_4_is_safe, true);

        let sample_vector_5 = vec![1, 3, 6, 7, 9];
        let sample_5_is_safe = is_safe(&sample_vector_5);
        assert_eq!(sample_5_is_safe, true);
    }

    #[test]
    fn extra_tests_work() {
        let sample_vector_0 = vec![1, 4, 3, 7, 8];
        let sample_0_is_safe = is_safe(&sample_vector_0);
        assert_eq!(sample_0_is_safe, true);

        let sample_vector_1 = vec![5, 4, 5, 6, 7];
        let sample_1_is_safe = is_safe(&sample_vector_1);
        assert_eq!(sample_1_is_safe, true);

        let sample_vector_2 = vec![1, 2, 3, 4, 9];
        let sample_2_is_safe = is_safe(&sample_vector_2);
        assert_eq!(sample_2_is_safe, true);

        let sample_vector_3 = vec![2, 3, 4, 5, 3];
        let sample_3_is_safe = is_safe(&sample_vector_3);
        assert_eq!(sample_3_is_safe, true);

        let sample_vector_4 = vec![8, 5, 6, 4, 3];
        let sample_4_is_safe = is_safe(&sample_vector_4);
        assert_eq!(sample_4_is_safe, true);

        let sample_vector_5 = vec![91, 92, 95, 93, 94];
        let sample_5_is_safe = is_safe(&sample_vector_5);
        assert_eq!(sample_5_is_safe, true);

        let sample_vector_6 = vec![40, 41, 43, 44, 47, 46, 47, 49];
        let sample_6_is_safe = is_safe(&sample_vector_6);
        assert_eq!(sample_6_is_safe, true);
    }
}
