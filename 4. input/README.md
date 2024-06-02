# Input

This directory contains examples demonstrating how to handle user input in Rust using the `std::io` module.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with examples of handling user input.

#### Directory Structure

```
input
│   Cargo.toml
|   Cargo.lock
└── src
    L main.rs
```

#### `Cargo.toml`

```toml
[package]
name = "input"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust
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

```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd input
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the output prompting for user input, reading the input, and displaying messages based on the input provided.

---

This example demonstrates how to handle user input in Rust using the `std::io` module. Continue exploring other topics to deepen your understanding of Rust programming.
