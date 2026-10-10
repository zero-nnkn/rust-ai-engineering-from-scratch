//! Phase 0 · Lesson 01 — Verify your dev environment
//!
//! Run: cargo run -p verify-env
//!
//! Checks the tools this curriculum uses. Required tools must pass now;
//! optional ones are only needed in later phases. Save the output as your
//! first piece of evidence.

use std::process::{Command, ExitCode};

struct Check {
    name: &'static str,
    cmd: &'static str,
    args: &'static [&'static str],
    required: bool,
    needed_for: &'static str,
    fix: &'static str,
}

const CHECKS: &[Check] = &[
    Check {
        name: "rustc",
        cmd: "rustc",
        args: &["--version"],
        required: true,
        needed_for: "everything",
        fix: "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
    },
    Check {
        name: "cargo",
        cmd: "cargo",
        args: &["--version"],
        required: true,
        needed_for: "everything",
        fix: "installed with rustup",
    },
    Check {
        name: "clippy",
        cmd: "cargo",
        args: &["clippy", "--version"],
        required: true,
        needed_for: "every lesson (linting)",
        fix: "rustup component add clippy",
    },
    Check {
        name: "rustfmt",
        cmd: "cargo",
        args: &["fmt", "--version"],
        required: true,
        needed_for: "every lesson (formatting)",
        fix: "rustup component add rustfmt",
    },
    Check {
        name: "git",
        cmd: "git",
        args: &["--version"],
        required: true,
        needed_for: "tracking your progress",
        fix: "install git from your package manager",
    },
    Check {
        name: "python3",
        cmd: "python3",
        args: &["--version"],
        required: false,
        needed_for: "Phase 12 (PyO3) and comparisons",
        fix: "install Python 3.10+",
    },
    Check {
        name: "maturin",
        cmd: "maturin",
        args: &["--version"],
        required: false,
        needed_for: "Phase 12 (Python packages)",
        fix: "pip install maturin",
    },
    Check {
        name: "docker",
        cmd: "docker",
        args: &["--version"],
        required: false,
        needed_for: "Phase 08 (deployment)",
        fix: "install Docker Desktop or docker engine",
    },
];

/// Run a command and return the first line of its output, or None if it failed.
fn probe(cmd: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    Some(text.lines().next().unwrap_or("").trim().to_string())
}

fn main() -> ExitCode {
    println!("Rust for AI Engineers · environment check\n");
    let mut missing_required = 0;

    for c in CHECKS {
        match probe(c.cmd, c.args) {
            Some(version) => println!("  ✓ {:<8} {}", c.name, version),
            None if c.required => {
                missing_required += 1;
                println!(
                    "  ✗ {:<8} MISSING (required: {})\n      fix: {}",
                    c.name, c.needed_for, c.fix
                );
            }
            None => println!(
                "  · {:<8} not found (optional, {})\n      fix: {}",
                c.name, c.needed_for, c.fix
            ),
        }
    }

    if missing_required == 0 {
        println!("\nAll required tools found. Save this output as your first evidence.");
        ExitCode::SUCCESS
    } else {
        println!("\n{missing_required} required tool(s) missing. Fix them, then run again.");
        ExitCode::FAILURE
    }
}
