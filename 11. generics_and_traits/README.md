# Generics and Traits

This directory contains examples demonstrating how to use generics and traits in Rust. Generics allow for writing flexible and reusable code, while traits define shared behavior that types must implement.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with examples of generics and traits.

#### Directory Structure

```
generics-and-traits
│   Cargo.toml
|   Cargo.lock
└── src
    L   main.rs
```

#### `Cargo.toml`

```toml
[package]
name = "generics-and-traits"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust

// Generics allow you to write code that works with any type.
fn print_items<T: std::fmt::Display>(items: &[T]) {
    for item in items {
        println!("{}", item);
    }
}

// Traits are a way to define a set of methods that a type must have.
trait Greet {
    fn greet(&self) -> String;
}

struct Customer {
    name: String,
}
impl Greet for Customer {
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.name)
    }
}

struct Employee {
    name: String,
}
impl Greet for Employee {
    fn greet(&self) -> String {
        format!("Hello, I'm the Employee, my name is {}", self.name)
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    println!("numbers: {:?}", numbers);
    print_items(&numbers);

    let words = ["hello", "world", "rust"];
    println!("words: {:?}", words);
    print_items(&words);
    println!("Hello, world!");

    let customer = Customer {
        name: "Alice".to_string(),
    };

    let employee = Employee {
        name: "Bob".to_string(),
    };

    println!("{}", customer.greet());
    println!("{}", employee.greet());
}

```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd generics-and-traits
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the following output:

```
numbers: [1, 2, 3, 4, 5]
1
2
3
4
5
words: ["hello", "world", "rust"]
hello
world
rust
Hello, I'm Alice
Hello, I'm the Employee, my name is Bob
```

---

This example demonstrates how to use generics to write flexible functions and how to define and implement traits for shared behavior in Rust. Continue exploring other topics to deepen your understanding of Rust programming.
