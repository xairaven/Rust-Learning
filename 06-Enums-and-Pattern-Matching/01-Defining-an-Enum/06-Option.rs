fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    // let absent = None;

    println!("{:?}", absent_number);
}