pub(crate) mod puzzle01a;
pub(crate) mod puzzle01b;

#[cfg(test)]
mod tests {
    use crate::puzzles::puzzle01;

    #[test]
    fn puzzle01a_test() {
        let result = puzzle01::puzzle01a::run();
        assert_eq!(result, 1151792);
    }

    #[test]
    fn puzzle01b() {
        let result = puzzle01::puzzle01b::run();
        assert_eq!(result, 21790168);
    }
}
