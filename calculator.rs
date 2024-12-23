use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero.");
                0.0 
            }
        }
    }
}
fn main() {
    let mut input = String::new();
    println!("Enter the first number: ");
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().expect("Invalid number");
    input.clear();
    println!("Enter the operation (+, -, *, /): ");
    io::stdin().read_line(&mut input).unwrap();
    let operator = input.trim();
    input.clear();
    println!("Enter the second number: ");
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = input.trim().parse().expect("Invalid number");

    let operation = match operator {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operation.");
            return; }};

    let result = calculate(operation);
    println!("The result is: {}", result);
}
