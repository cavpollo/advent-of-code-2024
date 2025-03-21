use regex::Regex;
use crate::puzzles::Puzzle;
use crate::tools::file;

pub struct Puzzle03b;

impl Puzzle03b {
    fn parse_operations(operations_string: &str, active: &mut bool) -> i32 {
        let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to parse the regex string");
        let do_regex = Regex::new(r"do\(\)").expect("Failed to parse the regex string");
        let dont_regex = Regex::new(r"don't\(\)").expect("Failed to parse the regex string");
        let full_regex = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").expect("Failed to parse the regex string");

        let mut cumulative_result= 0;
        for operation_capture in full_regex.captures_iter(operations_string) {
            let captured_text = &operation_capture[0];
            // print!("active {:?} captured: {:?} ", active, captured_text);

            if mul_regex.is_match(captured_text) {
                if !*active {
                    // println!("- SKIP");
                    continue
                }

                let value_0 = &operation_capture[1].parse::<i32>().expect("Failed to parse the integer");
                let value_1 = &operation_capture[2].parse::<i32>().expect("Failed to parse the integer");

                // println!("- parsed {:?} {:?}", value_0, value_1);

                cumulative_result += value_0 * value_1;
            } else if dont_regex.is_match(captured_text) {
                // println!("- DEACTIVATED");
                *active = false;
            } else if do_regex.is_match(captured_text) {
                // println!("- ACTIVATED");
                *active = true
            } else {
                panic!("Unknown parsed the regex string {:?}", captured_text);
            }
        }

        // println!("cumulative_result {:?}", cumulative_result);

        cumulative_result
    }
}

impl Puzzle for Puzzle03b {
    fn run() -> i32 {
        let mut cumulative_result= 0;
        let mut active = true;

        let buffer_lines = file::get_puzzle_buffer_lines(3);
        for read_line in buffer_lines {
            let line = read_line.expect("Failed to read the line from stdin");
            if line.trim().is_empty() {
                break;
            }

            let result = Puzzle03b::parse_operations(&line, &mut active);
            cumulative_result += result;

            // println!("--- LINE BREAK");
        }

        cumulative_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests_work() {
        let mut active_0 = true;
        let sample_0_string = "mul(44,46)";
        let sample_0_result = Puzzle03b::parse_operations(&sample_0_string, &mut active_0);
        assert_eq!(sample_0_result, 2024);
        assert_eq!(active_0, true);

        let mut active_1 = true;
        let sample_1_string = "mul(4*";
        let sample_1_result = Puzzle03b::parse_operations(&sample_1_string, &mut active_1);
        assert_eq!(sample_1_result, 0);
        assert_eq!(active_1, true);

        let mut active_2 = true;
        let sample_2_string = "mul(6,9!";
        let sample_2_result = Puzzle03b::parse_operations(&sample_2_string, &mut active_2);
        assert_eq!(sample_2_result, 0);
        assert_eq!(active_2, true);

        let mut active_3 = true;
        let sample_3_string = "?(12,34)";
        let sample_3_result = Puzzle03b::parse_operations(&sample_3_string, &mut active_3);
        assert_eq!(sample_3_result, 0);
        assert_eq!(active_3, true);

        let mut active_4 = true;
        let sample_4_string = "mul ( 2 , 4 )";
        let sample_4_result = Puzzle03b::parse_operations(&sample_4_string, &mut active_4);
        assert_eq!(sample_4_result, 0);
        assert_eq!(active_4, true);

        let mut active_5 = true;
        let sample_5_string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let sample_5_result = Puzzle03b::parse_operations(&sample_5_string, &mut active_5);
        assert_eq!(sample_5_result, 161);
        assert_eq!(active_5, true);

        let mut active_6 = true;
        let sample_6_string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let sample_6_result = Puzzle03b::parse_operations(&sample_6_string, &mut active_6);
        assert_eq!(sample_6_result, 48);
        assert_eq!(active_6, true);
    }

    #[test]
    fn extra_tests_work() {
        let mut active_0 = true;
        let sample_0_string = "mul(2,3)xmult(4,5)rmul(6,7)";
        let sample_0_result = Puzzle03b::parse_operations(&sample_0_string, &mut active_0);
        assert_eq!(sample_0_result, 48);
        assert_eq!(active_0, true);

        let sample_0_string = "mul(2,3)don't()mut(4,5)mul(6,7)";
        let sample_0_result = Puzzle03b::parse_operations(&sample_0_string, &mut active_0);
        assert_eq!(sample_0_result, 6);
        assert_eq!(active_0, false);

        let sample_0_string = "mul(2,3)xmult(4,5)do()mul(6,7)";
        let sample_0_result = Puzzle03b::parse_operations(&sample_0_string, &mut active_0);
        assert_eq!(sample_0_result, 42);
        assert_eq!(active_0, true);
    }
}
