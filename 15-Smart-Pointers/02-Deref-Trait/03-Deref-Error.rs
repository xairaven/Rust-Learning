struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y= MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/*
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src\main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^
 */