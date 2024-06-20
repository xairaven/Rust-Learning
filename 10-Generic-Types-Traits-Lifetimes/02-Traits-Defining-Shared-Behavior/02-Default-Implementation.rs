pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}