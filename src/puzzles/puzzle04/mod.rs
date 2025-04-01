pub mod puzzle04a;

#[cfg(test)]
mod tests {
    use crate::puzzles::Puzzle;
    use crate::puzzles::puzzle04::puzzle04a::Puzzle04a;

    #[test]
    fn puzzle04a_test() {
        let result = Puzzle04a::run();
        assert_eq!(result, 1);
    }

    // #[test]
    // fn puzzle04b() {
    //     let result = Puzzle04b::run();
    //     assert_eq!(result, 21790168);
    // }
}
