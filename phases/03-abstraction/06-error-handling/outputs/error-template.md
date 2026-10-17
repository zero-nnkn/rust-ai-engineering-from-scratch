# Error handling templates

## Library error: std only

```rust
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum MyError {
    Io { path: std::path::PathBuf, source: std::io::Error },
    Invalid { field: &'static str, reason: String },
    NotFound(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io { path, .. } => write!(f, "cannot read {}", path.display()),
            MyError::Invalid { field, reason } => write!(f, "invalid `{field}`: {reason}"),
            MyError::NotFound(what) => write!(f, "{what} not found"),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}
```

## Library error: thiserror

```rust
// cargo add thiserror   (check the current version)
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("cannot read {}", path.display())]
    Io { path: std::path::PathBuf, #[source] source: std::io::Error },

    #[error("invalid `{field}`: {reason}")]
    Invalid { field: &'static str, reason: String },

    #[error("{0} not found")]
    NotFound(String),

    // Automatic conversion for `?`, when you don't need extra fields:
    #[error("bad JSON")]
    Json(#[from] serde_json::Error),
}
```

## Application: std only

```rust
use std::error::Error;

fn report(err: &dyn Error) -> String {
    let mut out = err.to_string();
    let mut cause = err.source();
    while let Some(c) = cause {
        out.push_str(&format!("\n  caused by: {c}"));
        cause = c.source();
    }
    out
}

fn run() -> Result<(), Box<dyn Error>> {
    // ... use ? freely ...
    Ok(())
}

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {}", report(e.as_ref()));
            std::process::ExitCode::FAILURE
        }
    }
}
```

## Application: anyhow

```rust
// cargo add anyhow   (check the current version)
use anyhow::{bail, Context, Result};

fn run(path: &std::path::Path) -> Result<()> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    if text.is_empty() {
        bail!("{} is empty", path.display());
    }
    Ok(())
}

fn main() -> Result<()> {          // prints the error and each "Caused by:" on failure
    run(std::path::Path::new("input.txt"))
}
```

## Rules of thumb
1. **Libraries:** a specific enum; one variant per case a caller could handle differently.
2. **Applications:** one catch-all type (`anyhow::Error` / `Box<dyn Error>`) plus context.
3. Keep causes as `source`, don't stringify them. `Display` describes only this layer.
4. Add context where you know what you were trying to do: "loading config from X".
5. No `unwrap()`/`expect()` on input from outside the process. `panic!` is for bugs only.
6. Use `map_err` when the conversion needs extra information (which key, which file); `From` when it doesn't.
