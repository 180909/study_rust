fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("username: {}, email: {}, sign_in_count: {}, active: {}", user1.username, user1.email, user1.sign_in_count, user1.active);

    user1.email = String::from("anotheremail@example.com");
    println!("username: {}, email: {}, sign_in_count: {}, active: {}", user1.username, user1.email, user1.sign_in_count, user1.active);
}
