use std::io;

fn main() {
    println!("=== Rust Calculator ===");

    // First number
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input).unwrap();

    let num1: f64 = input.trim().parse().unwrap();

    // Operator
    input.clear();

    println!("Enter operator (+ - * /):");
    io::stdin().read_line(&mut input).unwrap();

    // Create an owned String so we can clear `input`
    let operator = input.trim().to_string();

    // Second number
    input.clear();

    println!("Enter second number:");
    io::stdin().read_line(&mut input).unwrap();

    let num2: f64 = input.trim().parse().unwrap();

    // Calculate result
    let result = match operator.as_str() {
        "+" => num1 + num2,

        "-" => num1 - num2,

        "*" => num1 * num2,

        "/" => {
            if num2 == 0.0 {
                println!("❌ Cannot divide by zero!");
                return;
            }

            num1 / num2
        }

        _ => {
            println!("❌ Invalid operator!");
            return;
        }
    };

    println!("Result: {}", result);
}