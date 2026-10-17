//! Phase 02 · Lesson 08 — Borrow checker battle patterns
//!
//! Run:  cargo run -p borrow-checker-patterns
//! Test: cargo test -p borrow-checker-patterns
//!
//! Seven situations where the borrow checker says no, and the idiomatic fix for
//! each. Every fix compiles without clone() or Rc<RefCell<..>>. The "fights" are
//! described in comments; Practice has you write each one and read the error.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Task {
    id: u32,
    prompt: String,
    done: bool,
    retries: u32,
}

fn task(id: u32, prompt: &str) -> Task {
    Task {
        id,
        prompt: prompt.into(),
        done: false,
        retries: 0,
    }
}

// ---- Pattern 1: modify a collection while iterating over it ---------------------
// Fight:  for t in &tasks { if t.done { tasks.remove(..) } }   → E0502
// Fix:    use the method made for it (retain), or collect what to change first.
fn drop_finished(tasks: &mut Vec<Task>) {
    tasks.retain(|t| !t.done);
}

fn add_followups(tasks: &mut Vec<Task>) {
    // Two phases: read (collect new tasks), then write (extend).
    let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let followups: Vec<Task> = tasks
        .iter()
        .filter(|t| t.prompt.ends_with('?'))
        .enumerate()
        .map(|(i, t)| {
            task(
                next_id + i as u32,
                &format!("verify answer to: {}", t.prompt),
            )
        })
        .collect();
    tasks.extend(followups);
}

// ---- Pattern 2: keep a reference into a Vec, then push to it ---------------------
// Fight:  let first = &tasks[0]; tasks.push(..); use(first)       → E0502
//         (push may reallocate and move every element: `first` would dangle)
// Fix:    keep an index (or an id) instead of a reference.
fn push_and_report(tasks: &mut Vec<Task>, new: Task) -> String {
    let first_idx = 0;
    tasks.push(new);
    format!("first is still #{}", tasks[first_idx].id)
}

// ---- Pattern 3: two &mut into the same Vec ---------------------------------------
// Fight:  let a = &mut v[i]; let b = &mut v[j];                   → E0499
// Fix:    split_at_mut proves the two parts don't overlap (or swap / get_disjoint_mut).
fn move_retries(tasks: &mut [Task], from: usize, to: usize) {
    assert!(from != to && from < tasks.len() && to < tasks.len());
    let (lo, hi) = (from.min(to), from.max(to));
    let (left, right) = tasks.split_at_mut(hi);
    let (a, b) = (&mut left[lo], &mut right[0]);
    let (src, dst) = if from < to { (a, b) } else { (b, a) };
    dst.retries += src.retries;
    src.retries = 0;
}

// ---- Pattern 4: a method borrows all of self, but you need two fields ------------
// Fight:  for t in self.tasks.iter_mut() { self.log(..) }         → E0499 / E0502
//         (self.log needs &mut self while self.tasks is mutably borrowed)
// Fix:    borrow the fields separately. The compiler tracks fields independently
//         inside one function body, but not across method calls.
struct Queue {
    tasks: Vec<Task>,
    log: Vec<String>,
}

impl Queue {
    fn retry_all(&mut self) {
        let log = &mut self.log; // borrow one field...
        for t in self.tasks.iter_mut() {
            // ...while mutating another
            t.retries += 1;
            log.push(format!("retry #{} ({})", t.id, t.retries));
        }
    }
}

// ---- Pattern 5: return a reference to something created inside the function -------
// Fight:  fn label(t: &Task) -> &str { &format!("#{}", t.id) }    → E0515
// Fix:    return an owned value. A reference must point at data that outlives the call.
fn label(t: &Task) -> String {
    format!("#{}: {}", t.id, t.prompt)
}

// ---- Pattern 6: move a value out of a struct you only have &mut to ----------------
// Fight:  let p = self.pending;   (behind &mut)                    → E0507
// Fix:    std::mem::take / replace / Option::take leave something valid behind.
struct Batch {
    pending: Vec<Task>,
}

impl Batch {
    fn drain_pending(&mut self) -> Vec<Task> {
        std::mem::take(&mut self.pending) // leaves an empty Vec in its place
    }
}

// ---- Pattern 7: graph / back-references ----------------------------------------------
// Fight:  struct Node { parent: &Node, children: Vec<Node> }  → lifetimes everywhere,
//         or Rc<RefCell<..>> and runtime panics
// Fix:    an arena: all nodes in one Vec, links are indexes.
#[derive(Debug)]
struct Plan {
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Step {
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Plan {
    fn add(&mut self, name: &str, parent: Option<usize>) -> usize {
        let id = self.steps.len();
        self.steps.push(Step {
            name: name.into(),
            parent,
            children: vec![],
        });
        if let Some(p) = parent {
            self.steps[p].children.push(id);
        }
        id
    }

    fn path(&self, mut id: usize) -> Vec<&str> {
        let mut names = vec![self.steps[id].name.as_str()];
        while let Some(p) = self.steps[id].parent {
            names.push(&self.steps[p].name);
            id = p;
        }
        names.reverse();
        names
    }
}

fn main() {
    let mut tasks = vec![
        task(1, "what is a lifetime?"),
        task(2, "summarize chapter 4"),
        task(3, "why is Rc not Send?"),
    ];
    tasks[1].done = true;

    drop_finished(&mut tasks);
    println!(
        "1 retain     : {:?}",
        tasks.iter().map(|t| t.id).collect::<Vec<_>>()
    );
    add_followups(&mut tasks);
    println!(
        "1 two-phase  : {:?}",
        tasks.iter().map(|t| t.id).collect::<Vec<_>>()
    );

    println!(
        "2 index      : {}",
        push_and_report(&mut tasks, task(9, "extra"))
    );

    tasks[0].retries = 3;
    move_retries(&mut tasks, 0, 2);
    println!(
        "3 split      : retries {:?}",
        tasks.iter().map(|t| t.retries).collect::<Vec<_>>()
    );

    let mut q = Queue {
        tasks: tasks.clone(), // a copy just for this demo, so `tasks` stays usable below
        log: vec![],
    };
    q.retry_all();
    println!("4 fields     : {}", q.log.join(", "));

    println!("5 owned      : {}", label(&tasks[0]));

    let mut batch = Batch { pending: tasks };
    let taken = batch.drain_pending();
    println!(
        "6 take       : took {}, left {}",
        taken.len(),
        batch.pending.len()
    );

    let mut plan = Plan { steps: vec![] };
    let root = plan.add("answer question", None);
    let search = plan.add("search docs", Some(root));
    let read = plan.add("read top hit", Some(search));
    plan.add("draft answer", Some(root));
    println!("7 arena      : {}", plan.path(read).join(" > "));
    let counts: HashMap<&str, usize> = plan
        .steps
        .iter()
        .map(|s| (s.name.as_str(), s.children.len()))
        .collect();
    println!(
        "7 arena      : root has {} children",
        counts["answer question"]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn followups_only_for_questions() {
        let mut v = vec![task(1, "a?"), task(2, "b")];
        add_followups(&mut v);
        assert_eq!(v.len(), 3);
        assert_eq!(v[2].id, 3);
        assert!(v[2].prompt.contains("a?"));
    }

    #[test]
    fn move_retries_works_in_both_directions() {
        let mut v = vec![task(1, "a"), task(2, "b"), task(3, "c")];
        v[2].retries = 5;
        move_retries(&mut v, 2, 0);
        assert_eq!((v[0].retries, v[2].retries), (5, 0));
        move_retries(&mut v, 0, 1);
        assert_eq!((v[0].retries, v[1].retries), (0, 5));
    }

    #[test]
    fn take_leaves_empty_vec() {
        let mut b = Batch {
            pending: vec![task(1, "x")],
        };
        assert_eq!(b.drain_pending().len(), 1);
        assert!(b.pending.is_empty());
    }

    #[test]
    fn arena_path_walks_parents() {
        let mut p = Plan { steps: vec![] };
        let a = p.add("a", None);
        let b = p.add("b", Some(a));
        let c = p.add("c", Some(b));
        assert_eq!(p.path(c), ["a", "b", "c"]);
        assert_eq!(p.steps[a].children, [b]);
    }
}
