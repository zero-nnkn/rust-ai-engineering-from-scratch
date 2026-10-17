# Python → Rust cheat sheet

## Syntax
| Python | Rust |
|---|---|
| `x = 5` | `let x = 5;` (immutable) |
| `x = 5; x += 1` | `let mut x = 5; x += 1;` |
| `def add(a: int, b: int) -> int:` | `fn add(a: i64, b: i64) -> i64 {` |
| `return a + b` | `a + b` (last expression, no `;`) |
| `if x > 0: ... elif ...: ... else: ...` | `if x > 0 { } else if ... { } else { }` |
| `y = a if c else b` | `let y = if c { a } else { b };` |
| `for x in xs:` | `for x in &xs {` |
| `for i in range(10):` | `for i in 0..10 {` |
| `while cond:` | `while cond {` |
| `while True:` | `loop {` |
| `f"{name} {score:.2f}"` | `format!("{name} {score:.2}")` |
| `print(x)` / `repr(x)` | `println!("{x}")` / `println!("{x:?}")` |
| `# comment` / `"""docstring"""` | `// comment` / `/// doc comment` |
| `pass` | `{}` |
| `assert x == y` | `assert_eq!(x, y);` |

## Data
| Python | Rust |
|---|---|
| `int` | `i32`, `i64`, `u32`, `usize` (sizes and indexes) … |
| `float` | `f64` (`f32` for ML tensors) |
| `bool` | `bool` |
| `str` | `String` (owned) / `&str` (borrowed view) |
| `bytes` | `Vec<u8>` / `&[u8]` |
| `list[T]` | `Vec<T>` / `&[T]` (borrowed view) |
| `tuple` | `(A, B)` |
| `dict[K, V]` | `HashMap<K, V>` (`BTreeMap` for sorted keys) |
| `set[T]` | `HashSet<T>` |
| `collections.deque` | `VecDeque<T>` |
| `Optional[T]` | `Option<T>`: `Some(v)` / `None` |
| `@dataclass` | `#[derive(Debug, Clone, PartialEq)] struct` |
| `Enum` + per-case data | `enum` with variants carrying fields |
| `None` return | `()` (the unit type) |

## Behaviour
| Python | Rust |
|---|---|
| `raise ValueError(...)` | `return Err(...)` |
| `try: f() except E: ...` | `match f() { Ok(v) => ..., Err(e) => ... }` |
| let exceptions propagate | `f()?` |
| `with open(p) as f:` | `let f = File::open(p)?;` (closed when `f` goes out of scope) |
| `[f(x) for x in xs if p(x)]` | `xs.iter().filter(\|x\| p(x)).map(\|x\| f(x)).collect::<Vec<_>>()` |
| generator / `yield` | an `Iterator` (lazy) |
| `lambda x: x + 1` | `\|x\| x + 1` |
| `isinstance` / duck typing | traits + generics, or `match` on an enum |
| class inheritance | composition + traits |
| `copy.deepcopy(x)` | `x.clone()` |
| shared mutable object | `Rc<RefCell<T>>` (one thread) / `Arc<Mutex<T>>` (threads): rarely the best design |
| `threading` + GIL | `std::thread` / `rayon`, with no GIL |
| `asyncio` | `tokio` |
| `pip install x` | `cargo add x` |
| `pytest` | `cargo test` |
| `ruff` / `black` / `mypy` | `cargo clippy` / `cargo fmt` / the compiler |
