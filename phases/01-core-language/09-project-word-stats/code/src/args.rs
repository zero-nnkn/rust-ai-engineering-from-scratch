//! Command-line parsing by hand, with std only. (Phase 05 replaces this with `clap`.)

use std::fmt;

/// Where to read the text from.
#[derive(Debug, PartialEq)]
pub enum Input {
    File(String),
    Stdin,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub input: Input,
    pub top: usize,
    pub chars_per_token: f64,
}

#[derive(Debug, PartialEq)]
pub enum UsageError {
    HelpRequested,
    MissingValue(&'static str),
    BadNumber { flag: &'static str, value: String },
    UnknownFlag(String),
    TooManyInputs,
}

impl fmt::Display for UsageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsageError::HelpRequested => write!(f, "{USAGE}"),
            UsageError::MissingValue(flag) => write!(f, "{flag} needs a value"),
            UsageError::BadNumber { flag, value } => {
                write!(f, "{flag} expects a positive number, got {value:?}")
            }
            UsageError::UnknownFlag(flag) => write!(f, "unknown option {flag}"),
            UsageError::TooManyInputs => write!(f, "give at most one input file"),
        }
    }
}

pub const USAGE: &str = "\
usage: word-stats [FILE] [--top N] [--chars-per-token X]

Reads FILE (or standard input if FILE is missing or '-') and prints word and
token statistics.

  --top N               show the N most frequent words (default 5)
  --chars-per-token X   characters per token for the estimate (default 4.0)
  -h, --help            show this help";

/// Parse arguments (without the program name).
pub fn parse_args<I>(args: I) -> Result<Config, UsageError>
where
    I: IntoIterator<Item = String>,
{
    let mut config = Config {
        input: Input::Stdin,
        top: 5,
        chars_per_token: 4.0,
    };
    let mut file: Option<String> = None;
    let mut iter = args.into_iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => return Err(UsageError::HelpRequested),
            "--top" => {
                let raw = iter.next().ok_or(UsageError::MissingValue("--top"))?;
                config.top = match raw.parse::<usize>() {
                    Ok(n) if n > 0 => n,
                    _ => {
                        return Err(UsageError::BadNumber {
                            flag: "--top",
                            value: raw,
                        })
                    }
                };
            }
            "--chars-per-token" => {
                let raw = iter
                    .next()
                    .ok_or(UsageError::MissingValue("--chars-per-token"))?;
                config.chars_per_token = match raw.parse::<f64>() {
                    Ok(x) if x > 0.0 && x.is_finite() => x,
                    _ => {
                        return Err(UsageError::BadNumber {
                            flag: "--chars-per-token",
                            value: raw,
                        })
                    }
                };
            }
            "-" => {}
            flag if flag.starts_with('-') => return Err(UsageError::UnknownFlag(arg)),
            _ => {
                if file.is_some() {
                    return Err(UsageError::TooManyInputs);
                }
                file = Some(arg);
            }
        }
    }
    if let Some(path) = file {
        config.input = Input::File(path);
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Config, UsageError> {
        parse_args(args.iter().map(|s| s.to_string()))
    }

    #[test]
    fn defaults_to_stdin() {
        let c = parse(&[]).unwrap();
        assert_eq!(c.input, Input::Stdin);
        assert_eq!(c.top, 5);
    }

    #[test]
    fn file_and_flags_in_any_order() {
        let c = parse(&["--top", "3", "notes.txt"]).unwrap();
        assert_eq!(c.input, Input::File("notes.txt".into()));
        assert_eq!(c.top, 3);
    }

    #[test]
    fn bad_values_are_errors() {
        assert_eq!(
            parse(&["--top", "0"]),
            Err(UsageError::BadNumber {
                flag: "--top",
                value: "0".into()
            })
        );
        assert_eq!(parse(&["--top"]), Err(UsageError::MissingValue("--top")));
        assert_eq!(
            parse(&["--verbose"]),
            Err(UsageError::UnknownFlag("--verbose".into()))
        );
        assert_eq!(parse(&["a", "b"]), Err(UsageError::TooManyInputs));
    }
}
