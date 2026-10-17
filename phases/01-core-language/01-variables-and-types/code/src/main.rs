//! Phase 01 · Lesson 01 — Variables, mutability & scalar types
//!
//! Run:  cargo run -p variables-and-types
//! Test: cargo test -p variables-and-types
//!
//! Scenario: budgeting tokens for an LLM call. Small numbers, but every
//! choice of type (signed or not, how many bits, integer or float) matters.

/// The most tokens any model in this example accepts.
const MAX_CONTEXT: u32 = 200_000;

/// How many tokens are left after the prompt, or None if the prompt doesn't fit.
fn remaining(context: u32, prompt_tokens: u32) -> Option<u32> {
    context.checked_sub(prompt_tokens)
}

/// Estimate cost in US dollars for a number of tokens at a price per million tokens.
fn cost_usd(tokens: u32, usd_per_million: f64) -> f64 {
    f64::from(tokens) / 1_000_000.0 * usd_per_million
}

fn main() {
    // 1. Immutable by default. `let` is like a Python variable you promise not to reassign.
    let prompt_tokens = 1_500; // no annotation: inferred as u32 from how it is used below
    let max_output: u32 = 4_096; // explicit type annotation

    // 2. `mut` opts in to change.
    let mut used: u32 = 0;
    used += prompt_tokens;
    used += max_output;
    println!("used          : {used} of {MAX_CONTEXT}");

    // 3. Shadowing: a *new* variable with the same name. Common when converting types.
    let budget = "8000"; // &str, e.g. read from an env var
    let budget: u32 = budget.parse().unwrap_or(0); // now a u32
    println!("budget        : {budget}");

    // 4. Integer types have fixed sizes. Overflow is checked in debug builds (panic),
    //    so be explicit about what you want when it can happen.
    let small: u8 = 250;
    println!("checked_add   : {:?}", small.checked_add(10)); // None
    println!("saturating_add: {}", small.saturating_add(10)); // 255
    println!("wrapping_add  : {}", small.wrapping_add(10)); // 4

    // 5. Unsigned subtraction that would go negative is a classic bug.
    println!("remaining     : {:?}", remaining(MAX_CONTEXT, 150_000));
    println!("remaining     : {:?}", remaining(MAX_CONTEXT, 250_000));

    // 6. Floats: f64 is Python's float. Integers don't mix with floats implicitly.
    let price = 3.0; // f64
    println!("cost          : ${:.4}", cost_usd(used, price));

    // 7. `as` casts. They never fail, which also means they can silently lose data.
    let big: i64 = 300;
    let truncated = big as u8; // 300 doesn't fit in u8 → 44
    let negative_to_unsigned = -1_i32 as u32; // → 4294967295
    println!("as u8         : {big} → {truncated}");
    println!("-1 as u32     : {negative_to_unsigned}");
    println!("try_from      : {:?}", u8::try_from(big)); // the safe version

    // 8. char is a Unicode scalar value (4 bytes), not a byte.
    let c = 'ữ';
    println!("char          : {c} is {} bytes in UTF-8", c.len_utf8());

    // 9. Tuples and arrays: fixed size, known at compile time.
    let (input_price, output_price) = (3.0, 15.0);
    let last_three_latencies_ms: [u32; 3] = [120, 95, 210];
    let worst = last_three_latencies_ms.iter().max().copied().unwrap_or(0);
    println!("prices        : in ${input_price}, out ${output_price} per 1M");
    println!("worst latency : {worst} ms");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remaining_is_none_when_prompt_too_big() {
        assert_eq!(remaining(100, 101), None);
        assert_eq!(remaining(100, 40), Some(60));
    }

    #[test]
    fn cost_is_proportional() {
        let c = cost_usd(500_000, 3.0);
        assert!((c - 1.5).abs() < 1e-12);
    }

    #[test]
    fn as_truncates_but_try_from_refuses() {
        let big: i64 = 300;
        assert_eq!(big as u8, 44);
        assert!(u8::try_from(big).is_err());
    }

    #[test]
    #[should_panic]
    fn overflow_panics_in_debug_builds() {
        let x: u8 = 255;
        let y = std::hint::black_box(x) + 1; // black_box stops the compiler folding it
        println!("{y}");
    }
}
