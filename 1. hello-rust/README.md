# Hello, Rust

This directory contains a simple "Hello, World!" program to get you started with Rust.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with the "Hello, World!" program.

#### Directory Structure

```
hello-rust
│   Cargo.toml
|   Cargo.lock
└── src
    L main.rs
```

#### `Cargo.toml`

```toml
[package]
name = "hello-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust
fn main() {
    println!("Hello, world!");
}
```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd hello-rust
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the following output:

```
Hello, world!
```

---

This example demonstrates the basic setup and structure of a Rust project. Continue to the next sections to learn more about Rust's features and capabilities.
