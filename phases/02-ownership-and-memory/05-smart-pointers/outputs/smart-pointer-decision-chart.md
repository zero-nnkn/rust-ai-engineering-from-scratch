# Smart pointer decision chart

Start at the top. Stop at the first answer that fits.

```
Do you need anything beyond plain ownership + & / &mut?
│
├─ No ──────────────────────────────────────────────► T, &T, &mut T   (the default, by far)
│
├─ A type that contains itself (tree, expression, linked list)?
│     └──────────────────────────────────────────────► Box<T> in the recursive field
│
├─ A trait object you must own (Vec of different tool types)?
│     └──────────────────────────────────────────────► Box<dyn Trait>   (Phase 03 · Lesson 03)
│
├─ Several owners of the same value, and nobody is clearly "the" owner?
│   ├─ read-only, one thread ────────────────────────► Rc<T>
│   ├─ read-only, several threads ───────────────────► Arc<T>
│   ├─ must also mutate, one thread ─────────────────► Rc<RefCell<T>>
│   └─ must also mutate, several threads
│         ├─ a single number/flag ───────────────────► Arc<AtomicU64> / Arc<AtomicBool>
│         ├─ many readers, rare writers ─────────────► Arc<RwLock<T>>
│         └─ otherwise ──────────────────────────────► Arc<Mutex<T>>
│
├─ One owner, but a &self method must update something (counter, cache)?
│   ├─ Copy type (u32, bool…), one thread ───────────► Cell<T>
│   ├─ otherwise, one thread ────────────────────────► RefCell<T>
│   └─ several threads ──────────────────────────────► Mutex<T> / atomics
│
└─ Back-references (child → parent) with Rc?
      └──────────────────────────────────────────────► Weak<T> for the back-reference
```

## Before you use any of them, ask
1. Could one part of the code own the value and lend `&` to the others?
2. Could the value be passed through function arguments or returned, instead of shared?
3. Could threads send messages over a channel instead of sharing state? (Phase 06)
4. For graphs: could nodes live in a `Vec<Node>` and refer to each other by index?

## Costs
| Type | Cost |
|---|---|
| `Box<T>` | one heap allocation |
| `Rc<T>` / `Arc<T>` | a count update on every clone and drop (atomic for `Arc`) |
| `RefCell<T>` | a runtime check on every borrow; **panics** on violation |
| `Mutex<T>` | a lock on every access; can block, can deadlock, can be poisoned by a panic |

## Review smells (Part 2)
- `Arc<Mutex<_>>` in code with no threads or tasks.
- `Rc<RefCell<_>>` used to dodge a borrow error instead of fixing the design.
- `.lock().unwrap()` in a long-running service (a panic elsewhere poisons the lock and turns into a crash here).
- Holding a `MutexGuard` across an `.await` or a slow call.
