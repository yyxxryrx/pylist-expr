# pylist-expr

Write Python-style comprehensions in Rust with procedural macros.

```rust
use pylist_expr::{list, iter, set, dict};

let squares = list![x * x for x in (1..=5)];
assert_eq!(squares, vec![1, 4, 9, 16, 25]);

let evens = list![x for x in (0..10) if x % 2 == 0];
assert_eq!(evens, vec![0, 2, 4, 6, 8]);
```

## Macros

| Macro  | Collects into          |
|--------|------------------------|
| `list!`| `Vec<_>`               |
| `iter!`| lazy iterator          |
| `set!` | `HashSet<_>`           |
| `dict!`| `HashMap<K, V>`        |

## Syntax

```
list![ <expr> for <pattern> in <source> [if <filter>] [for ...] ]
```

- **`<expr>`** — the value expression for each output element
- **`for <pattern>`** — a variable or destructuring pattern like `(k, v)`
- **`in <source>`** — any expression implementing `IntoIterator`
- **`if <filter>`** — (optional) keep only elements where the filter is true
- Multiple `for` clauses create nested iteration (rightmost varies fastest)

### Dictionary Syntax

```
dict! { <key_expr>: <value_expr> for <pattern> in <source> [if <filter>] [for ...] }
```

## How It Works

Macros expand at compile time into standard iterator chains. The innermost loop uses `.map()`, outer loops use `.flat_map()`, and `if` filters use `.filter()`. The outermost closure avoids `move` to allow repeated iteration.

For example, `list![i * 2 for i in (0..10) if *i > 3]` expands to:

```rust
(0..10).filter(|i| *i > 3).map(|i| i * 2).collect::<Vec<_>>()
```

## Examples

### Destructuring tuples

```rust
let b = list![k * v for (k, v) in (1..=3).zip([1, 2, 4])];
assert_eq!(b, vec![1, 4, 12]);
```

### Lazy iterator

```rust
let s: String = iter!(c for c in "Hello Steve!".chars() if *c != ' ').collect();
assert_eq!(s, "HelloSteve!");
```

### HashSet comprehension

```rust
let s = set! { i * j for i in (1..=10) for j in (1..=10) };
```

### HashMap comprehension

```rust
let d = dict! { i: i for i in (1..=10) };
```

### Nested comprehension in dict value

```rust
let d = dict! { k: list![*i for i in b.iter() if *i != k] for k in (0..10) };
```

## Test & Run

```sh
cargo test
cargo run --example basic
```

## License

MIT
