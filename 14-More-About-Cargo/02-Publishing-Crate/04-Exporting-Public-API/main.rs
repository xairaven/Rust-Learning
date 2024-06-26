// use project::PrimaryColor;
use project::kinds::PrimaryColor;
// use project::mix;
use project::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}