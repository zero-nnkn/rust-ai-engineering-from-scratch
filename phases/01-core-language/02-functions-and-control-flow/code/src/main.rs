//! Phase 01 · Lesson 02 — Functions, expressions & control flow
//!
//! Run:  cargo run -p functions-and-control-flow
//! Test: cargo test -p functions-and-control-flow
//!
//! Scenario: a retry policy for flaky LLM API calls, the kind of code every
//! agent needs. Exponential backoff, a cap, and a decision per HTTP status.

/// Delay before retry number `attempt` (0-based): base * 2^attempt, capped.
fn backoff_ms(attempt: u32, base_ms: u64, cap_ms: u64) -> u64 {
    // `if` is an expression: it produces a value, so no ternary operator is needed.
    let factor = if attempt >= 20 { 1 << 20 } else { 1 << attempt };
    (base_ms * factor).min(cap_ms) // last expression, no `;` → this is the return value
}

/// What to do with a response, decided by its HTTP status code.
fn decide(status: u16) -> &'static str {
    // `match` on ranges: like a chain of `elif`s, but the compiler checks it covers everything.
    match status {
        200..=299 => "done",
        429 => "retry (rate limited)",
        500..=599 => "retry (server error)",
        400..=499 => "fail (client error)",
        _ => "fail (unexpected)",
    }
}

/// Simulate calls that return these statuses in order. Returns how many attempts it took,
/// or None if we gave up.
fn run_with_retries(statuses: &[u16], max_attempts: u32) -> Option<u32> {
    let mut attempt = 0;
    // `loop` + `break value`: the loop itself is an expression.
    let outcome = loop {
        if attempt >= max_attempts {
            break None;
        }
        let status = match statuses.get(attempt as usize) {
            Some(&s) => s,
            None => break None, // ran out of simulated responses
        };
        attempt += 1;
        match decide(status) {
            "done" => break Some(attempt),
            d if d.starts_with("retry") => continue,
            _ => break None,
        }
    };
    outcome
}

fn main() {
    // `for` over a range, like `for i in range(6)`.
    print!("backoff ms    :");
    for attempt in 0..6 {
        print!(" {}", backoff_ms(attempt, 250, 5_000));
    }
    println!();

    for status in [200, 429, 503, 404, 302] {
        println!("status {status}    : {}", decide(status));
    }

    println!(
        "retries       : {:?}",
        run_with_retries(&[503, 429, 200], 5)
    );
    println!(
        "gives up      : {:?}",
        run_with_retries(&[503, 503, 503], 2)
    );
    println!("client error  : {:?}", run_with_retries(&[400, 200], 5));

    // Blocks are expressions too: the value of a block is its last expression.
    let total_wait: u64 = {
        let mut sum = 0;
        for a in 0..3 {
            sum += backoff_ms(a, 250, 5_000);
        }
        sum
    };
    println!("wait before 4th attempt: {total_wait} ms");

    // Labeled break: leave an outer loop from an inner one.
    let grid = [[1, 2, 3], [4, 99, 6], [7, 8, 9]];
    let mut found = None;
    'rows: for (r, row) in grid.iter().enumerate() {
        for (c, &value) in row.iter().enumerate() {
            if value > 50 {
                found = Some((r, c));
                break 'rows;
            }
        }
    }
    println!("first > 50 at : {found:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_doubles_then_caps() {
        let delays: Vec<u64> = (0..6).map(|a| backoff_ms(a, 100, 1_000)).collect();
        assert_eq!(delays, [100, 200, 400, 800, 1_000, 1_000]);
    }

    #[test]
    fn backoff_does_not_overflow_for_huge_attempts() {
        assert_eq!(backoff_ms(1_000, 100, 30_000), 30_000);
    }

    #[test]
    fn decide_covers_classes() {
        assert_eq!(decide(204), "done");
        assert_eq!(decide(429), "retry (rate limited)");
        assert_eq!(decide(418), "fail (client error)");
    }

    #[test]
    fn retries_until_success() {
        assert_eq!(run_with_retries(&[500, 500, 200], 5), Some(3));
    }

    #[test]
    fn stops_at_max_attempts() {
        assert_eq!(run_with_retries(&[500, 500, 200], 2), None);
    }
}
