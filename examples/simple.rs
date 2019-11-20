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
