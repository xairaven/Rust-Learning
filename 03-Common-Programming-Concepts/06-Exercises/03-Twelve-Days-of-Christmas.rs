fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me", days[i]);

        for j in (0..=i).rev() {
            if i == 0 && j == 0 {
                println!("{}", gifts[j]);
            } else if j == 0 {
                println!("And {}", gifts[j]);
            } else {
                println!("{},", gifts[j]);
            }
        }

        println!();
    }
}