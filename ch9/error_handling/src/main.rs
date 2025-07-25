use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("{e:?}"),
            },
            _ => panic!("{error:?}")
        },
    };

    let mut text = String::new();
    greeting_file.read_to_string(&mut text).expect("can't read text to string");
    println!("text in file:\n{}", text.trim());
}
