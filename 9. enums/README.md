# Enums

This directory contains examples demonstrating how to define and use enums in Rust, including pattern matching and methods on enums. It also includes an example of the `Option` enum for safe division.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with examples of defining and using enums.

#### Directory Structure

```
enums
│   Cargo.toml
|   Cargo.lock
└── src
    L   main.rs
```

#### `Cargo.toml`

```toml
[package]
name = "enums"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Method call on enums
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("The Quit variant was called."),
            Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
            Message::Write(text) => println!("Writing message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to: RGB({}, {}, {})", r, g, b),
        }
    }
}

// Pattern matching with enums
fn process_message(message: Message) {
    match message {
        Message::Quit => println!("The Quit variant was called."),
        Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to: RGB({}, {}, {})", r, g, b),
    }
}

// Options
// Option<T> is a generic enum defined by the standard library.
// enum Option<T> {
//     Some(T),
//     None,
// }

fn safe_division(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {

    let quit = Message::Quit;
    let move_message = Message::Move { x: 1, y: 2 };
    let write = Message::Write(String::from("Hello"));
    let change_color = Message::ChangeColor(0, 0, 0);


    println!("\nMethod call on enums\n");

    quit.call();
    move_message.call();
    write.call();
    change_color.call();

    println!("\nPattern matching with enums\n");

    process_message(quit);
    process_message(move_message);
    process_message(write);
    process_message(change_color);


    println!("\nOptions\n");
    let result = safe_division(9.0, 3.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }

    let result = safe_division(9.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }

}

```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd enums
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the following output:

```
Method call on enums

The Quit variant was called.
Move to coordinates: (1, 2)
Writing message: Hello
Changing color to: RGB(0, 0, 0)

Pattern matching with enums

The Quit variant was called.
Move to coordinates: (1, 2)
Writing message: Hello
Changing color to: RGB(0, 0, 0)

Options

Result: 3
Cannot divide by zero
```

---

This example demonstrates how to define and use enums in Rust, including using methods on enums, pattern matching, and handling optional values with `Option`. Continue exploring other topics to deepen your understanding of Rust programming.
