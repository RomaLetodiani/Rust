use std::io;

fn main() {
    println!("Enter your name: ");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name: &str = name.trim();
    println!("Hello, {}!", name);
    
    println!("Enter your age: ");
    let mut age_input: String = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: u32 = age_input.trim().parse().expect("Enter a valid number");
    
    println!("You are {} years old", age);

    if age < 18 {
        println!("You are not welcome here!");
    } else {
        println!("Welcome to the club!");
    }
}
