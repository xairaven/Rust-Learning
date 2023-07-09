fn main() {
    let s = String::from("Hello world!");

    let len = s.len();

    let slice = &s[0..2];
    let slice = &s[..2];    // equals

    let slice = &s[3..len];
    let slice = &s[3..];    // equals

    let slice = &s[0..len];
    let slice = &s[..];     // equals
}