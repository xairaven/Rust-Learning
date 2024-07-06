unsafe fn dangerous() {}

fn main() {
    unsafe {
        dangerous();
    }
}