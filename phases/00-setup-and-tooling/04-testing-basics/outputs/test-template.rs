// Test template. Paste at the bottom of a module (unit tests) and adapt.
// For integration tests, put the functions in tests/<name>.rs without the module wrapper,
// and import your crate with `use my_crate::...;`.

#[cfg(test)]
mod tests {
    use super::*;

    // Shared test data: a plain helper function instead of a pytest fixture.
    fn sample_input() -> &'static str {
        "replace me"
    }

    #[test]
    fn happy_path() {
        let input = sample_input();
        let expected = todo!("expected value");
        assert_eq!(do_thing(input), expected, "input was {input:?}");
    }

    #[test]
    fn bad_input_is_an_error_not_a_panic() {
        assert!(do_thing_checked("").is_err());
    }

    // `?` instead of unwrap(): the test fails with the error if any step fails.
    #[test]
    fn multi_step() -> Result<(), Box<dyn std::error::Error>> {
        let parsed: u32 = "42".parse()?;
        assert_eq!(parsed, 42);
        Ok(())
    }

    // Only for bugs (violated preconditions), never for bad user input.
    #[test]
    #[should_panic(expected = "must be positive")]
    fn precondition_violation_panics() {
        requires_positive(0);
    }

    #[test]
    fn floats_compared_with_tolerance() {
        let got: f64 = 0.1 + 0.2;
        assert!((got - 0.3).abs() < 1e-9, "got {got}");
    }

    #[test]
    #[ignore = "slow: cargo test -- --ignored"]
    fn large_input() {
        // ...
    }
}
