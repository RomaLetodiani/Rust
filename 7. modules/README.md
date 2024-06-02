# Modules

This directory contains examples demonstrating how to use modules in Rust, including nested modules.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with examples of using modules.
- `src/library.rs`: Source file defining modules and nested modules.

#### Directory Structure

```
modules
│   Cargo.toml
|   Cargo.lock
└── src
    │   main.rs
    L   library.rs
```

#### `Cargo.toml`

```toml
[package]
name = "modules"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust
mod library;

fn main() {
mod library;

fn main() {
    library::members::member1();
    library::members::member2();
    library::members::special_members::special_member1();
    library::members::special_members::special_member2();
}
}
```

#### `library.rs`

```rust
pub mod members {
    pub fn member1() {
        println!("member1");
    }
    pub fn member2() {
        println!("member2");
    }

    pub mod special_members {
        pub fn special_member1() {
            println!("special_member1");
        }
        pub fn special_member2() {
            println!("special_member2");
        }
    }
}
```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd modules
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the following output:

```
member1
member2
special_member1
special_member2
```

---

This example demonstrates how to define and use modules and nested modules in Rust. Modules help in organizing code into separate namespaces, making it more maintainable and readable. Continue exploring other topics to deepen your understanding of Rust programming.
