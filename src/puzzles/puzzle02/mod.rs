pub(crate) mod puzzle02a;
pub(crate) mod puzzle02b;

#[cfg(test)]
mod tests {
    use crate::puzzles::puzzle02;

    #[test]
    fn puzzle02a() {
        let result = puzzle02::puzzle02a::run();
        assert_eq!(result, 624);
    }

    #[test]
    fn puzzle02b() {
        let result = puzzle02::puzzle02b::run();
        assert_eq!(result, 658);
    }
}
