# Ownership

This directory contains examples demonstrating Rust's ownership model, including borrowing and references.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with examples of ownership and borrowing.

#### Directory Structure

```
ownership
│   Cargo.toml
|   Cargo.lock
└── src
    L main.rs
```

#### `Cargo.toml`

```toml
[package]
name = "ownership"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(&s);

    println!("{s}");
}
// if you will not use borrow operator, the code will not compile
fn takes_ownership(some_string: &String) {
    println!("{some_string}");
}
```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd ownership
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the output demonstrating Rust's ownership and borrowing rules:

```
hello
hello
```

---

This example demonstrates Rust's ownership model, including borrowing and references. The `takes_ownership` function borrows a reference to the `String`, allowing the main function to continue using it afterwards. Continue exploring other topics to deepen your understanding of Rust programming.
