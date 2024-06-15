fn main() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    let i = &v[2];
    println!("{i}")
}

/*
error[E0425]: cannot find value `v` in this scope
 --> src\main.rs:8:14
  |
8 |     let i = &v[2];
  |              ^
  |
help: the binding `v` is available in a different scope in the same function
 --> src\main.rs:3:13
  |
3 |         let v = vec![1, 2, 3, 4];
 */