#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    // Cannot use v1_iter there, because .sum got ownership

    assert_eq!(total, 6);
}