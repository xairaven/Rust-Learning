fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_plus_one(x: i32) -> i32 {
    return x + 1;
}