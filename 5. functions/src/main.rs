use std::io;

fn main() {
    println!("Hello, world!");
    first_function();
    function_with_parameters(10, 20, true);
    let z = function_with_return_value(10, 20);
    println!("The sum of 10 and 20 is: {z}");

    let mut name  = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    
    greeting(&name);

    let mut fib_num = String::new();
    println!("Enter the fibonacci number you want to calculate: ");
    io::stdin().read_line(&mut fib_num).expect("Failed to read line");
    let fib_num: u32 = fib_num.trim().parse().expect("Enter a valid number");

    let fib_result = fibonacci(fib_num);
    println!("The fibonacci number at position {} is: {}", fib_num, fib_result);
}


fn first_function (){
    println!("This is my first function");
}

fn function_with_parameters (x: i32, y: i32, b: bool){
    if b {
        println!("The value of x is: {x}");
    } else{
        println!("The value of y is: {y}");
    }
}

fn function_with_return_value (x: i32, y: i32) -> i32 {
    x + y
}

fn greeting(name: &str) {
    println!("Hello, {}!", name.trim());
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = b;
        b = a + b;
        a = temp;
    }

    b
}