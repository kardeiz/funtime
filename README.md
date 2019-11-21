# funtime

[![Docs](https://docs.rs/funtime/badge.svg)](https://docs.rs/crate/funtime/)
[![Crates.io](https://img.shields.io/crates/v/funtime.svg)](https://crates.io/crates/funtime)

A small proc-macro helper to time every statement in a given function (or item method).

## Usage

```rust
#[funtime::timed]
fn foo(y: i32) -> i32 {
    let mut x = 1;
    let d = 1_000;
    x += d;
    x += y;
    x
}   

#[funtime::timed]
fn main() {
    foo(23);
}
```

Prints:

```
funtime start: `foo`
  took 1µs: `let mut x = 1 ;`
  took 5µs: `let d = 1_000 ;`
  took 2µs: `x += d ;`
  took 2µs: `x += y ;`
  took 3µs: `x`
funtime end: `foo` took 12µs
funtime start: `main`
  took 49µs: `foo (23) ;`
funtime end: `main` took 56µs
```

Current version: 0.3.0
