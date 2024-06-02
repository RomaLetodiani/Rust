#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn introduce(&self) {
        println!("My name is {} and my email is {}", self.username, self.email);
    }
}


// Tuple Structs
struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("Roma@Gmail.com"),
        username: String::from("roma"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("another"),
        email: user1.email.clone(),
        ..user1
    };

    println!("User1: {} {} {} {}", user1.username, user1.email, user1.active, user1.sign_in_count);
    println!("User2: {} {} {} {}", user2.username, user2.email, user2.active, user2.sign_in_count);
    user1.introduce();
    user2.introduce();

    // Print Structs
    println!("User1: {:?}", user1); // Debug

    let black = Color(0, 0, 0);

    println!("Black in rgb: {} {} {}", black.0, black.1, black.2);

}
