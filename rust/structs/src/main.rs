struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username:  String::from("JohnDoe"),
        email: String::from("john.doe@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("hacked.john.doe@gmail.com");
    let user2 = build_user(user1.email, String::from("hacker"));
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2 // specifies that the remaining fields not explicitly set should
                // have the same value as the fields in the given instance.
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
