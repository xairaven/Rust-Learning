fn main() {
    let vec = vec![1, 2, 3];
    let v1_iter = vec.iter()
        .map(|x| x + 1);

    // Map was not used, because of lazy iterators. We just got new Map iterator

    // "Collecting" result. Consuming adapter
    // Annotating final type is needed.
    let result: Vec<_> = v1_iter.collect();

    println!("{result:?}", )
    // 2, 3, 4
}