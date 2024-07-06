fn main() {
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{s:?}");
}

/*
error[E0382]: borrow of partially moved value: `s`
 --> src/main.rs:8:15
  |
4 |     if let Some(_s) = s {
  |                 -- value partially moved here
...
8 |     println!("{s:?}");
  |               ^^^^^ value borrowed here after partial move
  |
  = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: borrow this binding in the pattern to avoid moving the value
  |
4 |     if let Some(ref _s) = s {
  |                 +++
 */