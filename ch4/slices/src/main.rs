fn main() {
    let s = String::from("Hello World");

    // string slices (&str type). Note when using rust's range syntax
    // you can leave the start/end index empty to specify start/end of string/container
    // let _s_hello = &s[..5];
    // let _s_world = &s[6..];
    // let _s_full = &s[..];

    let s_first = first_word(&s);
    let s_second = second_word(&s);
    let s_literal = "HellUwU World";
    println!("{s_first} {s_second}");
    println!("{}", first_word(&s[2..7]));
    println!("{}", first_word(s_literal));
}

// note the &str type of the parameter, accepts Strings, string literals and slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
     let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[(i + 1)..];
        }
    }
    &s[..0]
} 
