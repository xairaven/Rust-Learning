pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}