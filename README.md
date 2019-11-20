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

Current version: 0.2.0
