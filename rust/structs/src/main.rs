struct User {
    _active: bool,
    _username: String,
    email: String,
    _sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        _active: true,
        _username:  String::from("JohnDoe"),
        email: String::from("john.doe@gmail.com"),
        _sign_in_count: 1,
    };

    user1.email = String::from("hacked.john.doe@gmail.com");
    let user2 = build_user(user1.email, String::from("hacker"));
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user2 // specifies that the remaining fields not explicitly set should
                // have the same value as the fields in the given instance.
    };

    let scale = 2;
    let rec1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!(
        "The are of the rectangle is {} square pixels.",
        area(&rec1)
    );

    println!("rec is {rec1:#?}");
    dbg!(&rec1);
}

// We use &Rectangle to borrow the value instead of taking ownership of it.
// Then the caller can continue using Rectangle in the main program.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        _active: true,
        _username: username,
        email: email,
        _sign_in_count: 1,
    }
}
