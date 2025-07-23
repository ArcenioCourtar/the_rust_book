use std::env;

// we doin some pig-latin today
// If the first letter of a word is a consonant it gets moved to the back and
// appended with -ay. Words that start with a vowel just have -hay appended
// first = irst-fay | apple = apple-hay
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("please pass additional arguments");
        std::process::exit(0);
    }

    let mut words: Vec<String> = Vec::new();
    for i in args.into_iter().skip(1) {
        for j in i.trim().split(" ") {
            words.push(j.to_string());
        }
    }
    
    // perform string modifications before pushing
    let mut translated: Vec<String> = Vec::new();
    for i in words {
        let mut tmp = i.clone();
        if ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"]
            .iter().any(|s| i.starts_with(*s)) {
            tmp.push_str("-hay");
        } else {
            let tmp2 = tmp.remove(0);
            tmp.push_str("-");
            tmp.push_str(&tmp2.to_string());
            tmp.push_str("ay");
        }
        translated.push(tmp);
    }

    for i in translated {
        print!("{i}");
        print!(" ");
    }
}
