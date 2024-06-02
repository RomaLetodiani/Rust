// Result<T, E> enum is defined in the standard library as follows:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn check_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        Err("password is too short".to_string())
    } else {
        Ok(())
    }
}

fn main() {
    let mut password = "1234567";
    println!("password: {}", password);
    match check_password(password) {
        Ok(()) => println!("password is strong enough"),
        Err(err) => println!("error: {}", err),
    }

    password = "12345678";
    println!("password: {}", password);
    match check_password(password) {
        Ok(()) => println!("password is strong enough"),
        Err(err) => println!("error: {}", err),
    }


}
