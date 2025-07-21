// regular struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

impl User {
    fn sign_in(&mut self) {
        self.sign_in_count += 1;
    }
    fn constructor() -> Self {  // "constructors in rust don't have
        Self {                  // mandatory naming rules
            active: false,
            username: String::from("0"),
            email: String::from("0"),
            sign_in_count: 0
        }
    }
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn print_user(user: &User) {
    println!("Username: {}\nEmail: {}\nActive: {}\nSign in count: {}",
        user.username, user.email, user.active, user.sign_in_count);
}

// tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);

// unit-like struct
struct _UnitStruct;

fn main() {
    // I havent used tuples yet so also doing that here
    let tuple = (10, 20, 30);
    println!("tuple {}", tuple.0);
    let color = Color(1, 2, 3);
    println!("color {}", color.0 + color.1 + color.2);

    let user1 = build_user("default".to_string(), "wow@domain.com".to_string());
    print_user(&user1);
    let _user2 = User {
        username: user1.username.clone(),
        email: user1.email.clone(),
        ..user1
    };
    println!();
    print_user(&user1); // compiles, explicitly copied the two strings
                        // so user1 still has ownership
    let user3 = User {
        ..user1
    };
    // print_user(&user1);  // will not compile, 
                            // values without the copy trait have been moved
    print_user(&user3);
    println!();

    let mut user4 = build_user("Mario".to_string(), "ihatebowser@mushroom.com".to_string());
    println!("sign in count: {}", user4.sign_in_count);
    user4.sign_in();
    println!("sign in count: {}", user4.sign_in_count);
    println!();
    let user5 = User::constructor();
    print_user(&user5);

    println!();
    // println!("{}", user3);   // Display trait is not implemented, println
                                // does not know hot to display all struct info
    // You can add a Debug trait to a struct to let println easily print it
    println!("{color:?}");
    // you can also use the dbg! macro
    dbg!(&color);
}
