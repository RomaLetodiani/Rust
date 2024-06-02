# Error Handling

This directory contains examples demonstrating how to handle errors in Rust using the `Result` enum.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with examples of error handling using `Result`.

#### Directory Structure

```
error-handling
│   Cargo.toml
|   Cargo.lock
└── src
    L   main.rs
```

#### `Cargo.toml`

```toml
[package]
name = "error-handling"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust
// Result<T, E> enum is defined in the standard library as follows:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn check_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        Err("password is too short".to_string())
    } else {
        Ok(())
    }
}

fn main() {
    let mut password = "1234567";
    println!("password: {}", password);
    match check_password(password) {
        Ok(()) => println!("password is strong enough"),
        Err(err) => println!("error: {}", err),
    }

    password = "12345678";
    println!("password: {}", password);
    match check_password(password) {
        Ok(()) => println!("password is strong enough"),
        Err(err) => println!("error: {}", err),
    }
}
```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd error-handling
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the following output:

```
password: 1234567
error: password is too short
password: 12345678
password is strong enough
```

---

This example demonstrates how to handle errors in Rust using the `Result` enum. The `check_password` function returns a `Result` indicating whether the password meets the criteria. The main function checks the result and prints an appropriate message. Continue exploring other topics to deepen your understanding of Rust programming.
