// Cargo.toml:
// rand = "0.8.5"

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}