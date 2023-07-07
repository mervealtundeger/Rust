use std::io; //This line imports the 'io' module from the standard library, which provides input/output functionality.

enum Operation {
    Add(f64, f64),             // Variant Add holds two f64 values for addition
    Subtract(f64, f64),        //Variant Subtract holds two f64 values for subtraction
    Multiply(f64, f64),        //Variant Multiply holds two f64 values for multiplication
    Divide(f64, f64),          //Variant Divide holds two f64 values for division
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,             // Perform addition
        Operation::Subtract(a, b) => a - b,        // Perform subtraction
        Operation::Multiply(a, b) => a * b,        // Perform multiplication
        Operation::Divide(a, b) => a / b,          // Perform division
    }
}

fn main() {
    println!("Calculator");

    println!("Enter the first number:");                             //Enter the first number
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Read error");           //Read error
    let num1: f64 = num1.trim().parse().expect("Invalid input");     //Invalid input"

    println!("Enter the operation (+, -, *, /):");
    let mut operation_choice = String::new();
    io::stdin()
        .read_line(&mut operation_choice)
        .expect("Read error");                    //Read error

    let operation = match operation_choice.trim() {
        "+" => Operation::Add,                    // Choose addition
        "-" => Operation::Subtract,               // Choose subtraction
        "*" => Operation::Multiply,               // Choose multiplication
        "/" => Operation::Divide,                 // Choose division
        _ => {
            println!("Invalid operation choice");            //Invalid action selection
            return;
        }
    };

    println!("Enter the second number:");                               //Enter the second number
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Read error");              //Read error
    let num2: f64 = num2.trim().parse().expect("Invalid input");        //Invalid login

    let result = calculate(operation(num1, num2));

    println!("Result: {}", result);
}
