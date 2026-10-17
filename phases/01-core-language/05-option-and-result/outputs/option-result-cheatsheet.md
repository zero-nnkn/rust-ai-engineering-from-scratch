# Option & Result cheat sheet

```rust
enum Option<T>    { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

## Propagate
| Code | Meaning |
|---|---|
| `let v = f()?;` | on `Err`/`None`, return it from the current function |
| `fn main() -> Result<(), Box<dyn std::error::Error>>` | lets you use `?` in `main` |

## Convert
| Have → Want | Method |
|---|---|
| `Option<T>` → `Result<T, E>` | `.ok_or(e)`, `.ok_or_else(\|\| e)` |
| `Result<T, E>` → `Option<T>` | `.ok()` (error is discarded: be sure you mean it) |
| `Result<T, E1>` → `Result<T, E2>` | `.map_err(\|e\| E2::from(e))` |
| `Option<Result<T, E>>` → `Result<Option<T>, E>` | `.transpose()` |
| `Option<&String>` → `Option<&str>` | `.as_deref()` (or `.map(String::as_str)`) |
| `&Option<T>` → `Option<&T>` | `.as_ref()` |

## Transform without unwrapping
| Method | Does |
|---|---|
| `.map(f)` | transform the `Some`/`Ok` value |
| `.and_then(f)` | chain another step that can fail (`f` returns Option/Result) |
| `.filter(p)` | `Some` → `None` if the predicate is false (Option) |
| `.or(other)` / `.or_else(f)` | fallback if `None`/`Err` |
| `.is_some()` / `.is_none()` / `.is_ok()` / `.is_err()` | check without taking the value |

## Extract
| Method | On `None` / `Err` |
|---|---|
| `match` / `if let` / `let ... else` | you decide |
| `.unwrap_or(v)` | `v` |
| `.unwrap_or_else(\|\| compute())` | computed fallback (lazy) |
| `.unwrap_or_default()` | type's default (`0`, `""`, empty Vec) |
| `.expect("reason")` | **panic**: only for invariants, with a reason |
| `.unwrap()` | **panic**: tests and throwaway code only |

## Rules
1. No `unwrap()`/`expect()` on external input: files, network, CLI, env, user input, LLM output.
2. A missing optional value is `Option`. A failed operation is `Result`. A bug is a panic.
3. Errors are values: make them an `enum`, implement `Display`, and test them with `assert_eq!`.
4. Prefer `?` and let the caller decide, unless this function really knows what to do.
