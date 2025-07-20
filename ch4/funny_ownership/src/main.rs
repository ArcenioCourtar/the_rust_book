fn main() {
    // Shallow copies of heap allocated data make
    // everything but the newest copy invalid.
    // Use clone to create a deep copy, and use references to pass data to a
    // function without taking ownership.
    let s1 = String::from("hello world!");
    let s2 = s1; //shallow copy, moves the ptr and takes ownership away from s1
    println!("s2: {s2}");
    // println!("s1: {s1}"); // this will not compile, trying to borrow a moved value
    let s3 = s2.clone();
    println!("s3: {s3}"); // still have access to s2 because cloning creates a deep copy

    func_borrow(&s2); // borrows the value with a ref, can use the original still
    println!("s2: {s2}");

    let s4 = func_take(s2); // function takes ownership. s2 can no longer be
                            // reached outside of this function. This function
                            // does return a String so you could potentially pass
                            // ownership along
    // println!("s2: {s2}");   // this will not compile, ownership
    println!("s4: {s4}");

    func_take(s4.clone()); // cloning passes a deep copy to the funct, does not 
                           // transfer ownership so you can still access s4
    println!("s4: {s4}");

    // Messing about with mutable references
    let mut s_mut = String::from("hello");
    println!("s_mut: {s_mut}");
    change(&mut s_mut); // this function can change the string it's referring to
    println!("s_mut: {s_mut}");

    // let ref1 = &mut s_mut;
    // let ref2 = &mut s_mut;
    // println!("{}, {}", ref1, ref2); // will not compile, cannot have 
    //                                 // mutiple mut borrows of the same value
}

fn change(str_mut: &mut String) {
    str_mut.push_str(", world");
}

fn func_borrow(str: &String) {
    println!("Borrowing str: {str}");
}

fn func_take(str: String) -> String {
    println!("Taking ownership of str: {str}");
    str
}
