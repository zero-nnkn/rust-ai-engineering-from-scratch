//! Phase 02 · Lesson 06 — Drop & RAII: resources that clean themselves up
//!
//! Run:  cargo run -p drop-and-raii
//! Test: cargo test -p drop-and-raii
//!
//! Two guards an agent runtime needs:
//!   Span        records "enter"/"exit" around a step, for tracing
//!   TempWorkspace  a scratch directory for a tool, deleted when done, even on error

use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};

/// A trace log shared by all spans in one run.
type Trace = RefCell<Vec<String>>;

/// RAII guard: "enter" is recorded when created, "exit" when dropped.
/// You can't forget to close a span, because closing is not a separate call.
struct Span<'t> {
    name: String,
    trace: &'t Trace,
}

impl<'t> Span<'t> {
    fn enter(trace: &'t Trace, name: &str) -> Self {
        trace.borrow_mut().push(format!("enter {name}"));
        Span {
            name: name.to_string(),
            trace,
        }
    }
}

impl Drop for Span<'_> {
    fn drop(&mut self) {
        self.trace.borrow_mut().push(format!("exit  {}", self.name));
    }
}

/// RAII guard for a temporary directory.
struct TempWorkspace {
    path: PathBuf,
}

impl TempWorkspace {
    fn create(name: &str) -> std::io::Result<Self> {
        let path = std::env::temp_dir().join(format!("raii-{name}-{}", std::process::id()));
        fs::create_dir_all(&path)?;
        Ok(TempWorkspace { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        // Drop can't return an error, so cleanup is best-effort. Here a failure to
        // delete a temp directory is not worth crashing for.
        if let Err(e) = fs::remove_dir_all(&self.path) {
            eprintln!("warning: could not remove {}: {e}", self.path.display());
        }
    }
}

/// A tool that writes intermediate files. The `?` can return early with an error,
/// and the workspace is still cleaned up.
fn run_tool(input: &str) -> std::io::Result<(usize, PathBuf)> {
    let ws = TempWorkspace::create("tool")?;
    let file = ws.path().join("input.txt");
    fs::write(&file, input)?;
    let text = fs::read_to_string(&file)?;
    if text.contains("FAIL") {
        return Err(std::io::Error::other("tool rejected the input"));
    }
    Ok((text.len(), ws.path().to_path_buf()))
    // `ws` is dropped here (and also on every early return above)
}

fn agent_step(trace: &Trace, fail: bool) -> Result<(), String> {
    let _step = Span::enter(trace, "agent_step"); // `_step`, not `_`: see Practice 2
    {
        let _llm = Span::enter(trace, "llm_call");
        // ... call the model ...
    } // _llm dropped here: "exit llm_call"
    let _tool = Span::enter(trace, "tool_call");
    if fail {
        return Err("tool failed".into()); // both open spans still close, in reverse order
    }
    Ok(())
}

fn main() {
    let trace = Trace::default();
    let _ = agent_step(&trace, false); // the trace is what we're showing; the Result is ignored on purpose
    let _ = agent_step(&trace, true);
    for line in trace.borrow().iter() {
        println!("trace   : {line}");
    }

    match run_tool("hello tool") {
        Ok((len, dir)) => println!(
            "tool ok : {len} bytes, workspace still exists: {}",
            dir.exists()
        ),
        Err(e) => println!("tool err: {e}"),
    }
    match run_tool("please FAIL") {
        Ok(_) => println!("tool ok"),
        Err(e) => println!("tool err: {e}"),
    }

    // Drop order: locals are dropped in reverse order of declaration;
    // struct fields and Vec elements in order.
    let order = Trace::default();
    {
        let _a = Span::enter(&order, "a");
        let _b = Span::enter(&order, "b");
        let v = vec![Span::enter(&order, "v0"), Span::enter(&order, "v1")];
        drop(v); // drop early, explicitly
        let _c = Span::enter(&order, "c");
    }
    let exits: Vec<String> = order
        .borrow()
        .iter()
        .filter(|l| l.starts_with("exit"))
        .map(|l| l.trim_start_matches("exit  ").to_string())
        .collect();
    println!("drops   : {}", exits.join(" → "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spans_close_in_reverse_order_even_on_error() {
        let trace = Trace::default();
        assert!(agent_step(&trace, true).is_err());
        assert_eq!(
            *trace.borrow(),
            [
                "enter agent_step",
                "enter llm_call",
                "exit  llm_call",
                "enter tool_call",
                "exit  tool_call",
                "exit  agent_step"
            ]
        );
    }

    #[test]
    fn workspace_is_removed_after_success_and_failure() {
        let (_, dir) = run_tool("ok").unwrap();
        assert!(!dir.exists(), "workspace should be gone after success");
        assert!(run_tool("FAIL").is_err());
    }

    #[test]
    fn drop_runs_during_a_panic() {
        let trace = Trace::default();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _s = Span::enter(&trace, "doomed");
            panic!("boom");
        }));
        assert!(result.is_err());
        assert_eq!(
            trace.borrow().last().map(String::as_str),
            Some("exit  doomed")
        );
    }
}
