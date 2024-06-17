fn main() {
    // let s1 = String::from("hello");
    // let h = s1[0];

    /*
    let hello = "Здравствуйте";
    let answer = &hello[0];

    println!("{answer}");
    */

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}