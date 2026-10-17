//! word-stats: the binary. Reads input, calls the library, prints, sets the exit code.
//!
//! Run:  cargo run -p project-word-stats -- phases/01-core-language/09-project-word-stats/code/samples/ownership.txt --top 3
//!       echo "some text" | cargo run -p project-word-stats
//! Test: cargo test -p project-word-stats

use project_word_stats::{parse_args, Input, TextStats, UsageError};
use std::io::Read;
use std::process::ExitCode;

fn read_input(input: &Input) -> Result<String, String> {
    match input {
        Input::File(path) => {
            std::fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))
        }
        Input::Stdin => {
            let mut text = String::new();
            std::io::stdin()
                .read_to_string(&mut text)
                .map_err(|e| format!("cannot read standard input: {e}"))?;
            Ok(text)
        }
    }
}

fn main() -> ExitCode {
    let config = match parse_args(std::env::args().skip(1)) {
        Ok(c) => c,
        Err(UsageError::HelpRequested) => {
            println!("{}", UsageError::HelpRequested);
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("error: {e}\n(run with --help for usage)");
            return ExitCode::from(2); // conventional exit code for usage errors
        }
    };

    let text = match read_input(&config.input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };

    print!("{}", text.stats(config.top, config.chars_per_token));
    ExitCode::SUCCESS
}
