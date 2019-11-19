#[funtime::timed]
fn foo() -> i32 {

    let mut x = 1;

    x += 1;

    10

}   


fn main() {
    foo();
}