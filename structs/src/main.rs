fn main() {
    let email = String::from("someone@example.com");
    let username = String::from("someusername123");
    let user1 = build_user(email, username);

    let user2 = User {
        email: String::from("someoneElse@example.com"),
        ..user1
    };

    //let username = user1.username;
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
