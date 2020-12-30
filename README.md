# mark_last

A simple extension to rust iterators which gives the next value as well as a boolean indicating if this is the last value of the iterator.

The iterator returned yields pairs `(b, val)`, where `b` is true if this is the last value and `val` is the value returned by the iterator.

### Usage
Add `mark_last = "0.9.2"` to the dependencies section of your Cargo.toml file, and use it like so:

```rust
use mark_last::MarkLastIterator;

let in_data = vec![1, 2, 3, 5, 99];

let out_data: Vec<_> = in_data
    .into_iter()
    .mark_last()
    .collect();

assert_eq!(
    out_data,
    vec![
        (false, 1),
        (false, 2),
        (false, 3),
        (false, 5),
        (true, 99)
    ]
)
```
