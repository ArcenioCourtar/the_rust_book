fn main() {
    let lines = ["A partridge in a pear tree", "Two turtle doves and", "Three french hens",
    "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming",
    "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping",
    "Eleven pipers piping", "Twelve drummers drumming"];
    let num_lines = ["first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut count = 0;

    for day_number in num_lines {
        println!("For the {day_number} day of Christmas my true love sent to me");
        for j in (0..=count).rev() {
            println!("{}", lines[j]);
        }
        println!();
        count += 1;
    }
}
