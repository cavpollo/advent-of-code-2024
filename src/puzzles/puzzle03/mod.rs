pub mod puzzle03a;

#[cfg(test)]
mod tests {
    use crate::puzzles::Puzzle;
    use crate::puzzles::puzzle03::puzzle03a::Puzzle03a;

    #[test]
    fn puzzle03a() {
        let result = Puzzle03a::run();
        assert_eq!(result, 173529487);
    }
}
