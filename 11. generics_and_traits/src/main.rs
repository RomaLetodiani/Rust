
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
