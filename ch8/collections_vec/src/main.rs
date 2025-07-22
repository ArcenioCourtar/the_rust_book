use std::env; // accepting additional arguments
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("please pass additional arguments");
        std::process::exit(0);
    }

    let mut nums = Vec::new();
    for i in args.into_iter().skip(1) {
        let tmp: i32 = i.trim().parse().expect("Inputs need to be valid numbers");
        nums.push(tmp); // idk how to do proper error handling yet so we crash if input isnt valid
    }

    nums.sort();
    println!("Median of number list: {}", nums[nums.len() / 2]);

    let mut occurences = HashMap::new();
    for i in nums {
        let count = occurences.entry(i).or_insert(0);
        *count += 1;
    }

    let mut most_occ = HashMap::new();
    let mut highest = 0;
    for (key, value) in occurences {
        if value == highest { most_occ.insert(key, value); }
        if value > highest {
            highest = value;
            most_occ.clear();
            most_occ.insert(key, value);
        }
    }

    println!("The most frequently occuring numbers occur {highest} times. list:");
    for key in most_occ.into_keys() {
        println!("{key}");
    }
    // I LOVE THE BORROW CHECKER THIS IS A GREAT WAY TO MANAGE MEMORY
    // unironically
    //
    // ALSO ALSO, look at how good the compiler is at inferring types!!!!
    // (After I fixed the errors I made along the way)
}
