#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let mut user = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("email@gmail.com"),
        sign_in_count: 1
    };

    user.username = String::from("username");

    let user_2 = build_user(String::from("email2@gmail.com"), String::from("user_2"));

    let user_3 = User {
        email: String::from("test@gmail.com"),
        ..user
    };


    // println!("{:?}", user);
    println!("{:?}", user_2);
    println!("{:?}", user_3);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}