pub mod puzzle04a;
pub mod puzzle04b;

#[cfg(test)]
mod tests {
    use crate::puzzles::Puzzle;
    use crate::puzzles::puzzle04::puzzle04a::Puzzle04a;
    use crate::puzzles::puzzle04::puzzle04b::Puzzle04b;

    #[test]
    fn puzzle04a_test() {
        let result = Puzzle04a::run();
        assert_eq!(result, 2599);
    }

    #[test]
    fn puzzle04b() {
        let result = Puzzle04b::run();
        assert_eq!(result, 1948);
    }
}
