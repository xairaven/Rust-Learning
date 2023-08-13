fn main() {
    // Entire instance must be mutable
    // Rust doesn't allow us to mark only certain fields as mutable.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}