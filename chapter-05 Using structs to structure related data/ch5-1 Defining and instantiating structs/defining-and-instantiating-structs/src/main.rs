fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Create a new struct from another without using update syntax.
    // NOTE:: Only the emial field changes
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another1@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Create a new struct using update syntax.
    let user3 = User {
        email: String::from("test@email.com"),
        ..user2
    };

    // display user details
    println!("user3 email: {}", user3.email);

    // Buid user from a function.
    let mail = String::from("test@example");
    let username = String::from("testusername");

    let mut another_user = build_user(mail, username);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
