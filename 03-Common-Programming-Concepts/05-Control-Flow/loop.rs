fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Again! {counter}");

        if counter == 10 {
            break;
        }
    }
}