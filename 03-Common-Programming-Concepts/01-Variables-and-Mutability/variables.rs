fn main() {
    let mut x = 5;
    // let x = 5 -- error (variables are immutable)
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}