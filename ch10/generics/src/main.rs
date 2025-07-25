fn main() {
    let number_list = vec![10, 20, 50, 60, 100];
    let char_list = vec!['a', 'p', 'z', 'f', 'k'];

    let result = largest(&number_list);
    println!("largest number: {result}");
    // the example code in the book binds the return value of largest()
    // to another instance of result. Is it bad practice to use
    // functions like I do below?
    println!("largest char: {}", largest(&char_list));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}


