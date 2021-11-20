#[cfg(test)]
mod compiler {
    use the_super_tiny_rusty_compiler::compile;

    #[test]
    fn should_success() {
        const INPUT: &str = "(add 2 (subtract 4 2))";
        const RESULT: &str = "add(2, subtract(4, 2))";
        assert_eq!(compile(INPUT), RESULT);
    }
}
