use std::io;

fn main() {
    println!("Please enter a temperature in Celsius!");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: u32 = celsius
        .trim()
        .parse()
        .expect("Temperature entered was not a number");

    let fahrenheit = {celsius as f32} * 9.0 / 5.0 + 32.0;

    println!("{celsius}° Celsius = {fahrenheit}° Fahrenheit!");
}