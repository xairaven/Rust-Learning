/*
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src\main.rs:2:11
    |
2   |     match x {
    |           ^ pattern `None` not covered
    |
note: `Option<i32>` defined here
   --> C:\Users\Alex\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib/rustlib/src/rust\library\core\src\option.rs:572:1
    |
572 | pub enum Option<T> {
    | ^^^^^^^^^^^^^^^^^^
...
576 |     None,
    |     ---- not covered
    = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
3   ~         Some(value) => Some(value + 1),
4   +         None => todo!()
    |
 */

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1)
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);

    println!("{six:?}");
}