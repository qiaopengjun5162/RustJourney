struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 0,
    }
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("acb@126.com"),
        username: String::from("abc"),
        active: false,
        sign_in_count: 0,
    };
}
