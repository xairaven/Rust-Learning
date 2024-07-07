fn generic<T>(t: T) {
    // --snip--
}

// Made by compiler automatically
fn generic<T: Sized>(t: T) {
    // --snip--
}

fn generic<T: ?Sized>(t: &T) {
    // --snip--
}