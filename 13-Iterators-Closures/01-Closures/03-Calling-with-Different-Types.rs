fn main() {
    let example_closure = |x| x;

    // Will work
    let s = example_closure(String::from("hello"));
    // Compile error, expected String
    let n = example_closure(5);
}