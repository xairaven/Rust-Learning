fn main() {
    let v: Vec<i32> = Vec::new();

    let v1 = vec![1, 2, 3];

    // cannot borrow as mutable
    // v1.push(5);

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}