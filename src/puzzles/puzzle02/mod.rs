pub mod puzzle02a;
pub mod puzzle02b;

#[cfg(test)]
mod tests {
    use crate::puzzles::Puzzle;
    use crate::puzzles::puzzle02::puzzle02a::Puzzle02a;
    use crate::puzzles::puzzle02::puzzle02b::Puzzle02b;

    #[test]
    fn puzzle02a() {
        let result = Puzzle02a::run();
        assert_eq!(result, 624);
    }

    #[test]
    fn puzzle02b() {
        let result = Puzzle02b::run();
        assert_eq!(result, 658);
    }
}
