# Vectors

This directory contains examples demonstrating how to create, manipulate, and access elements in vectors in Rust.

### Files

- `Cargo.toml`: Configuration file for the Rust project.
- `src/main.rs`: Main source file with examples of using vectors.

#### Directory Structure

```
vectors
│   Cargo.toml
|   Cargo.lock
└── src
    L   main.rs
```

#### `Cargo.toml`

```toml
[package]
name = "vectors"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `main.rs`

```rust
fn main() {

    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec);

    let mut vec2 = vec![4, 5, 6];

    println!("First Element of vec2 {:?}", vec2[0]);

    for i in &vec {
        println!("{}", i);
    }

    for i in &mut vec2 {
        *i += 50;
    }

    if let Some(third) = vec.get(2) {
        println!("Third element of vec: {}", third);
    }

    if let Some(fourth) = vec.get(3) {
        println!("Fourth element of vec: {}", fourth);
    } else {
        println!("No fourth element in vec");
    }

    let slice = &vec2[1..3];
    println!("Slice of &vec2[1..3]: {:?}", slice); // [55, 56]

    let slice = &vec2[1..];
    println!("Slice of &vec2[1..]: {:?}", slice); // [55, 56]

    let slice = &vec2[..2];
    println!("Slice of &vec2[..2]: {:?}", slice); // [54, 55]

    let slice = &vec2[..];
    println!("Slice of &vec2[..]: {:?}", slice); // [54, 55, 56]

    let slice = &vec2;
    println!("Slice of &vec2: {:?}", slice); // [54, 55, 56]

    let slice = &vec2[1..=2];
    println!("Slice &vec2[1..=2]: {:?}", slice); // [55, 56]

    // let slice = &vec2[1..=3];
    // println!("Slice &vec2[1..=3]: {:?}", slice); // Panic

    // pop returns an Option and removes the last element from the vector
    if let Some(last) = vec2.pop() {
        println!("Last element of vec2: {}", last);
    }

    println!("vec2: {:?}", vec2);
}
```

#### Running the Program

1. Navigate to this directory:

   ```sh
   cd vectors
   ```

2. Compile and run the program:
   ```sh
   cargo run
   ```

You should see the following output:

```
[1, 2, 3]
First Element of vec2 4
1
2
3
Third element of vec: 3
No fourth element in vec
Slice of &vec2[1..3]: [55, 56]
Slice of &vec2[1..]: [55, 56]
Slice of &vec2[..2]: [54, 55]
Slice of &vec2[..]: [54, 55, 56]
Slice of &vec2: [54, 55, 56]
Slice &vec2[1..=2]: [55, 56]
Last element of vec2: 56
vec2: [54, 55]
```

---

This example demonstrates how to create and manipulate vectors in Rust. It shows how to add elements, iterate over vectors, access elements, use slices, and handle the `Option` type returned by the `pop` method. Continue exploring other topics to deepen your understanding of Rust programming.
