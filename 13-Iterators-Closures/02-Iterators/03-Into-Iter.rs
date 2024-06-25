fn main() {
    let v1 = vec![1, 2, 3];

    // Getting ownership of elements
    let v1_iter = v1.into_iter();

    // Getting IterMut<i32>
    // ERROR, because we moved values before
    let v1_iter_mut = v1.iter_mut();
}