use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the first number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num2: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Choose an operation:\n1. Add\n2. Subtract\n3. Multiply\n4. Divide");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let operation: u32 = input.trim().parse().expect("Invalid input");

    let result = match operation {
        1 => num1 + num2,
        2 => num1 - num2,
        3 => num1 * num2,
        4 => {
            if num2 == 0.0 {
                panic!("Cannot divide by zero!");
            }
            num1 / num2
        }
        _ => panic!("Invalid operation"),
    };

    println!("Result: {}", result);
}
