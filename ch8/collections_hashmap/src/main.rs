use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to the staff database");

    let mut list: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("options: add, display, exit");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match input.trim() {
            "add" => add_employee(&mut list),
            "display" => {
                sort_list(&mut list);
                display_list(&list);
            }
            "exit" => {
                println!("shutting down");
                break;
            }
            _ => println!("please enter a valid command"),
        }
    }
}

fn add_employee(list: &mut HashMap<String, Vec<String>>) {
    let mut input: (String, String) = (String::new(), String::new());

    print!("department: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input.0)
        .expect("failed to read line");

    print!("employee name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input.1)
        .expect("failed to read line");

    let node = list
        .entry(String::from(input.0.trim()))
        .or_insert(Vec::new());
    node.push(input.1.trim().to_string());
}

fn display_list(list: &HashMap<String, Vec<String>>) {
    for (key, val) in list.iter() {
        println!("----\ndepartment: {key}");
        for i in val {
            println!("{i}");
        }
    }
}

fn sort_list(list: &mut HashMap<String, Vec<String>>) {
    for (_key, val) in list.iter_mut() {
        val.sort();
    }
}
