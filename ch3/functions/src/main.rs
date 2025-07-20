use std::io;

fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("failed to read line");

    println!("resulting number: {}", number(5, str));
}

fn number(num: i32, str: String) -> i32 {
    println!("passed string: {str}");
    if str.trim() == "zero" {
        return 0;
    }
    num + 1
}
