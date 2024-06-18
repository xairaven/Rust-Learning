/*
Convert strings to pig latin.
The first consonant of each word is moved to the end of the word and “ay” is added,
so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
(“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
 */

fn main() {
    let original = String::from("This is an example of Hog Latin. As you can see, it’s silly, but lots of fun for children.");

    let result = convert_to_pig_latin(&original);
    println!("{result}");
}

fn convert_to_pig_latin(original: &str) -> String {
    let vowels: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut result = String::new();

    for word in original.split_whitespace() {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            if vowels.contains(&first_char) {
                result.push_str(&format!("{}-hay ", word));
            } else {
                result.push_str(&format!("{}-{}ay ", chars.as_str(), first_char));
            }
        }
    }

    result.trim_end().to_string()
}