use regex::Regex;
use crate::puzzles::Puzzle;
use crate::tools::file;

pub struct Puzzle03a;

impl Puzzle03a {
    fn parse_operations(operations_string: &str) -> i32 {
        let regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to parse the regex string");

        let mut cumulative_result= 0;
        for operation_capture in regex.captures_iter(operations_string) {
            // print!("captured: {:?} ", &operation_capture[0]);

            let value_0 = &operation_capture[1].parse::<i32>().expect("Failed to parse the integer");
            let value_1 = &operation_capture[2].parse::<i32>().expect("Failed to parse the integer");

            // println!("- parsed {:?} {:?}", value_0, value_1);

            cumulative_result += value_0 * value_1;
        }

        // println!("cumulative_result {:?}", cumulative_result);

        cumulative_result
    }
}

impl Puzzle for Puzzle03a {
    fn run() -> i32 {
        let mut cumulative_result= 0;

        let buffer_lines = file::get_puzzle_buffer_lines(3);
        for read_line in buffer_lines {
            let line = read_line.expect("Failed to read the line from stdin");
            if line.trim().is_empty() {
                break;
            }

            let result = Puzzle03a::parse_operations(&line);
            cumulative_result += result;
        }

        cumulative_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests_work() {
        let sample_0_string = "mul(44,46)";
        let sample_0_result = Puzzle03a::parse_operations(&sample_0_string);
        assert_eq!(sample_0_result, 2024);

        let sample_1_string = "mul(4*";
        let sample_1_result = Puzzle03a::parse_operations(&sample_1_string);
        assert_eq!(sample_1_result, 0);

        let sample_2_string = "mul(6,9!";
        let sample_2_result = Puzzle03a::parse_operations(&sample_2_string);
        assert_eq!(sample_2_result, 0);

        let sample_3_string = "?(12,34)";
        let sample_3_result = Puzzle03a::parse_operations(&sample_3_string);
        assert_eq!(sample_3_result, 0);

        let sample_4_string = "mul ( 2 , 4 )";
        let sample_4_result = Puzzle03a::parse_operations(&sample_4_string);
        assert_eq!(sample_4_result, 0);

        let sample_5_string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let sample_5_result = Puzzle03a::parse_operations(&sample_5_string);
        assert_eq!(sample_5_result, 161);
    }

    #[test]
    fn extra_tests_work() {
        let sample_0_string = "mul(2,3)xmult(4,5)rmul(6,7)";
        let sample_0_result = Puzzle03a::parse_operations(&sample_0_string);
        assert_eq!(sample_0_result, 48);
    }
}
