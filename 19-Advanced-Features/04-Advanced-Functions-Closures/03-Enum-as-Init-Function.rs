#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    dbg!(list_of_statuses);
}

/*
[src/main.rs:10:5] list_of_statuses = [
    Value(0),Value(1),Value(2),Value(3),Value(4),Value(5),
    Value(6),Value(7),Value(8),Value(9),Value(10),Value(11),
    Value(12),Value(13),Value(14),Value(15),Value(16),Value(17),
    Value(18),Value(19),
]
 */