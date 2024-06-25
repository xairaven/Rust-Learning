#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // Mut because we need to mutate iterator
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}