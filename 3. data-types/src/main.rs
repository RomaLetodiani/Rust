fn main() {
    // Integer types
    // Signed integer types
    let q: isize = 5; // isize is the default integer type for the current architecture (e.g., 64-bit or 32-bit)
    let x: i8 = 5;    // 8-bit signed integer
    let y: i16 = 5;   // 16-bit signed integer
    let z: i32 = 5;   // 32-bit signed integer
    let o: i64 = 5;   // 64-bit signed integer
    let p: i128 = 5;  // 128-bit signed integer

    /* 
    Signed Integers (i)
    Signed integers can represent both positive and negative values.
    They use one bit to store the sign (positive or negative) 
    and the remaining bits to store the magnitude of the number.

    Types: i8, i16, i32, i64, i128, isize
    Ranges:
    i8: -128 to 127
    i16: -32,768 to 32,767
    i32: -2,147,483,648 to 2,147,483,647
    i64: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    i128: -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727
    isize: Depends on the architecture (32-bit or 64-bit)   
    */

    // Unsigned integer types
    let a: usize = 5; // usize is the default unsigned integer type for the current architecture
    let b: u8 = 5;    // 8-bit unsigned integer
    let c: u16 = 5;   // 16-bit unsigned integer
    let d: u32 = 5;   // 32-bit unsigned integer
    let e: u64 = 5;   // 64-bit unsigned integer
    let f: u128 = 5;  // 128-bit unsigned integer

    /*
    Unsigned Integers (u)
    Unsigned integers can only represent non-negative values.
    They use all bits to store the magnitude of the number, 
    allowing them to represent a larger positive range 
    compared to signed integers of the same bit size.

    Types: u8, u16, u32, u64, u128, usize
    Range:
    u8: 0 to 255
    u16: 0 to 65,535
    u32: 0 to 4,294,967,295
    u64: 0 to 18,446,744,073,709,551,615
    u128: 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455
    usize: Depends on the architecture (32-bit or 64-bit)
    */

    // Floating point types
    let g: f32 = 5.0; // 32-bit floating point
    let h: f64 = 5.0; // 64-bit floating point (default)

    // Boolean types
    let i: bool = true;
    let j: bool = false;

    // Character types
    let k: char = 'a'; // char represents a single Unicode scalar value

    // Additional examples
    let heart: char = '❤'; // Unicode heart symbol

    // String slice type
    let m: &str = "Hello, Rust!"; // &str is a string slice type

    // Array type
    let n: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 elements of type i32

    // Tuple type
    let o: (i32, f64, char) = (42, 6.4, 'z'); // Tuple with mixed types

    // Unit type
    let p: () = (); // Unit type, used for functions that don't return a value
}
