#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side
        }
    }
}

fn main() {
    let square = Rectangle::square(5);

    dbg!(square);
}