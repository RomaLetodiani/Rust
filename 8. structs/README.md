# Structs

This directory contains examples demonstrating how to define and use structs in Rust, including tuple structs and methods associated with structs.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with examples of defining and using structs.

#### Directory Structure

```
structs
│   Cargo.toml
|   Cargo.lock
└── src
    L   main.rs
```

#### `Cargo.toml`

```toml
[package]
name = "structs"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust
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

```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd structs
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the following output:

```
User1: roma Roma@Gmail.com true 1
User2: another Roma@Gmail.com true 1
My name is roma and my email is Roma@Gmail.com
My name is another and my email is Roma@Gmail.com
User1: User { username: "roma", email: "Roma@Gmail.com", sign_in_count: 1, active: true }
Black in rgb: 0 0 0
```

---

This example demonstrates how to define and use structs in Rust, including defining methods on structs and using tuple structs. Continue exploring other topics to deepen your understanding of Rust programming.
