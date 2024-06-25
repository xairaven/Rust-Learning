fn main() {
    let v1 = vec![1, 2, 3];

    // Iter<i32>
    let v1_iter = v1.iter();

    // &i32
    for val in v1_iter {
        println!("{val}");
    }


    // i32
    for val in v1 {
        println!("{val}");
    }
}