fn main() {
    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!();

    // while loops
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    println!();

    // for loops
    let a = [1, 2, 3, 4, 5];
    for elem in a {
        println!("the value is: {elem}");
    }
    println!();
    for number in (1..=6).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
