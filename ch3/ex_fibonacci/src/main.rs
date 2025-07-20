use std::io::{self, Write};

fn main() {
    print!("Input which fibonacci number to generate (I start indexing at 0): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line"); // will crash if this happens

    let input: u32 = match input.trim().parse() {
        Ok(input) => input,
        Err(_) => {
            println!("hand me a positive number please.");
            std::process::exit(0);
        }
    };

    let mut fib: [u32; 3] = [0, 1, 0];
    for _i in 0..input {
        fib[2] = fib[1];
        fib[1] = fib[0];
        fib[0] = fib[1] + fib[2];
        }

    println!("Fibonacci number at index {input} is {}", fib[0]); // compiler complains putting the
                                                                 // fib[0] inside the {}
}
