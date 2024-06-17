fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s1 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s1}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s1 = s1 + "-" + &s2 + "-" + &s3;
    println!("{s1}");
}