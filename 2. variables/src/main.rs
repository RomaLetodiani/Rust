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
