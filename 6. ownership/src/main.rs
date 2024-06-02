fn main() {
    let s = String::from("hello");
    takes_ownership(&s);

    println!("{s}");
}
// if you will not use borrow operator, the code will not compile
fn takes_ownership(some_string: &String) {
    println!("{some_string}");
}