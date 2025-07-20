use std::io;

fn main() {
    let mut temp = String::new();
    println!("Please pass Fahrenheit temp");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line"); // I want to crash if the line can't be read

    let temp: i32 = match temp.trim().parse() { //shadowing is funny
        Ok(temp) => temp,
        Err(_) => {
            println!("NUMBER PLS");
            std::process::exit(0);
        }
    };

    if temp < -459 {
        println!("A bit too cold chief");
        std::process::exit(0);
    }
    
    println!("temperature in Fahrenheit: {temp}");
    println!("temperature in Celsius: {}", (temp - 32) * 5 / 9);

    // another way of getting Celsius. More shadowing!
    let temp = (temp - 32) * 5 / 9;
    println!("temperature in Celsius: {temp}");
}
