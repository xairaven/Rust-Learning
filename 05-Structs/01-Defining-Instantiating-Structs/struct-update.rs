fn main() {
    let user1 = build_user(String::from("Alex@ukr.net"), String::from("xairaven"));

    let user2_dull = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        username: String::from("wow"),
        email: String::from("wow@gmail.com"),
        ..user1
    };

    println!("{}", user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}