fn main() {
    let mut s = String::from("Hello World!");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src\main.rs:6:5
  |
4 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
5 |
6 |     s.clear(); // error!
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("the first word is: {}", word);
  |                                       ---- immutable borrow later used here
 */