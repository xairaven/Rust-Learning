use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // May outlive borrowed value `list`
    // Not working
    thread::spawn(|| println!("From thread: {list:?}"))
        .join()
        .unwrap();


    println!("After spawning thread: {list:?}");
}