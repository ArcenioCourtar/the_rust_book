use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    println!("Welcome to the staff database");

    let mut list: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("options: add, display, exit"); io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match input.trim() {
            "add" => add_employee(&mut list),
            "display" => display_list(&mut list),
            "exit" => { println!("shutting down"); break; },
            _ => println!("please enter a valid command"),
        }
    }
}

fn add_employee(list: &mut HashMap<String, Vec<String>>) {
    let mut input: (String, String) = (String::new(), String::new());

    print!("department: "); io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input.0)
        .expect("failed to read line");

    print!("employee name: "); io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input.1)
        .expect("failed to read line");

    list.entry(String::from(input.0.trim()))
        .or_insert(vec![input.1.trim().to_string()]);
}

// should probably split this up in two functions.
// Do the sorting in another function to retain mutability here
fn display_list(list: &mut HashMap<String, Vec<String>>) {
    for (key, val) in list.iter_mut() {
        println!("department: {key}");
        val.sort();
        for i in val {
            println!("{i}");
        }
    }
}
