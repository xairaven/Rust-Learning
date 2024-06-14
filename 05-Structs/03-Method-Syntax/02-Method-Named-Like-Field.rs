#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    if rectangle.width() {
        println!("The rectangle has a nonzero width; it is {}", rectangle.width);
    }
}