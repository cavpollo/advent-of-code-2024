pub mod puzzle03a;
pub mod puzzle03b;

#[cfg(test)]
mod tests {
    use crate::puzzles::Puzzle;
    use crate::puzzles::puzzle03::puzzle03a::Puzzle03a;
    use crate::puzzles::puzzle03::puzzle03b::Puzzle03b;

    #[test]
    fn puzzle03a() {
        let result = Puzzle03a::run();
        assert_eq!(result, 173529487);
    }

    #[test]
    fn puzzle03b() {
        let result = Puzzle03b::run();
        assert_eq!(result, 99532691);
    }
}
