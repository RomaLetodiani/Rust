# Variables

This directory contains examples demonstrating variable declarations, mutability, and shadowing in Rust.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with examples of variables.

#### Directory Structure

```
variables
│   Cargo.toml
|   Cargo.lock
└── src
    L main.rs
```

#### `Cargo.toml`

```toml
[package]
name = "variables"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust
fn main() {
    // Variables are immutable by default
    let x: i32 = 5;
    println!("The value of immutable x is {x}!");

    // This will not work
    // x = 6;

    // This will work
    let x = 6;  // This is called shadowing

    let mut y: i32 = 5;
    println!("The value of mutable y is {y} at first!");
    y = 6;
    println!("The value of shadowed x is {x}!");
    println!("The value of y is {y} after mutation!");

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {MAX_POINTS}!");
}
```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd variables
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the output demonstrating the use of variables, mutability, shadowing, and constants.

---

This example demonstrates the use of variables, mutability, shadowing, and constants in Rust. Continue exploring other topics to deepen your understanding of Rust programming.
