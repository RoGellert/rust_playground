struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main () {
    let mut user1 = User {
        active: true,
        username: String::from("some_name123"),
        email: String::from("user1_email1@gmail.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user2_email@gmail.com"),
        ..user1
    };
    
    println!("{}", user1.active);
    println!("{}", user2.active);
    println!("{}", user1.email);
    println!("{}", user2.email);

    user1.email = String::from("another_user1_email@gmail.com");
    println!("{}", user1.email);
}