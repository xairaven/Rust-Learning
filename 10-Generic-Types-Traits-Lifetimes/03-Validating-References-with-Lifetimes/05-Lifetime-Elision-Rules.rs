// Works because of Elision rules
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Elision Rules:
/*
1. Compiler assigns a lifetime parameter to each parameter that’s a reference.
In other words, a function with one parameter gets one lifetime parameter:
fn foo<'a>(x: &'a i32);

a function with two parameters gets two separate lifetime parameters:
fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

2. If there is exactly one input lifetime parameter, that lifetime is assigned to
all output lifetime parameters:
fn foo<'a>(x: &'a i32) -> &'a i32.

3. If there are multiple input lifetime parameters, but one of them is &self or &mut self
because this is a method, the lifetime of self is assigned to all output lifetime parameters.
This third rule makes methods much nicer to read and write because fewer symbols are necessary.
 */

// Initial.
fn first_word(s: &str) -> &str {}

// First and second rules applied -- so, no inferred lifetimes needed:
fn first_word<'a>(s: &'a str) -> &'a str {}


// Initial
fn longest(x: &str, y: &str) -> &str {}
// First rule
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
// Second rule
// ???
// So, we need inferred lifetimes.