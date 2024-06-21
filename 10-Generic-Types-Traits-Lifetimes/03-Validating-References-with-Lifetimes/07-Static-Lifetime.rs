// 'static denotes that the affected reference
// can live for the entire duration of the program.
fn main() {
    // All str literals have static lifetime
    let s: &'static str = "I have a static lifetime.";
    // Same:
    let s: &str = "Same string";
}