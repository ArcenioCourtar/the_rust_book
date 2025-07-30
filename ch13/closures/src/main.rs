use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };

    // you can have multiple immutable references, so no borrow checker bs
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("Print from closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // can't have multiple mutable refs, can't print list in main until
    // closure is called and no longer used
    let mut list = vec![4, 5, 6];
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    // println!("Before calling closure: {list:?}"); // compiler angy
    borrows_mutably();
    println!("After calling closure: {list:?}");

    // using move keyword to pass ownership to the thread. compiler complains
    // otherwise. If you don't move the reference it can't be guaranteed
    // it's still valid when the thread tries to print
    let list = vec![7, 8, 9];
    println!("Before closure and thread: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // println!("After thread: {list:?}"); // compiler sad

    println!("number {}", expensive_closure(32));

    let example_closure = |x| x;
    let s = example_closure(String::from("hello world!"));
    // let n = example_closure(5); // will not compile
    //                          // Compiler inferred String type from prev line
    println!("{}", s);
}
