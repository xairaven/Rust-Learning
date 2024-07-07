fn bar() -> ! {
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue, // possible because of never
    };
}