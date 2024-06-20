// Different types
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// Same type
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// + syntax:
pub fn notify<T: Summary + Display>(item: &T) {}