pub mod puzzle01a;
pub mod puzzle01b;

#[cfg(test)]
mod tests {
    use crate::puzzles::Puzzle;
    use crate::puzzles::puzzle01::puzzle01a::Puzzle01a;
    use crate::puzzles::puzzle01::puzzle01b::Puzzle01b;

    #[test]
    fn puzzle01a_test() {
        let result = Puzzle01a::run();
        assert_eq!(result, 1151792);
    }

    #[test]
    fn puzzle01b() {
        let result = Puzzle01b::run();
        assert_eq!(result, 21790168);
    }
}
