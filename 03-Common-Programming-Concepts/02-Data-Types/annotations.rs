fn main() {
    // needs annotation because type is unknown
    let guess: u32 = "42".parse().expect("Not a number!");
}