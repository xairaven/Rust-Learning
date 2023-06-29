use std::io;

fn main() {
    println!("Please enter an index of Fibonacci number.");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n
        .trim()
        .parse()
        .expect("Temperature entered was not a number");

    println!("Nth number of Fibonacci sequence = {}", fibonacci_number(n));
}

fn fibonacci_number(n: i32) -> i32 {
    if n <= 0 {
        panic!("{n} is not right argument!");
    } else if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    }

    let mut sum = 0;
    let mut a = 0;
    let mut b = 1;
    for _i in 3..n+1{
        sum = a + b;
        a = b;
        b = sum;
    }
    sum
}