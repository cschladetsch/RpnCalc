use std::io::{self, Write};
use std::str::FromStr;

/// Helper function to pop two operands from the stack
fn pop_two_operands(stack: &mut Vec<f64>, operator: &str) -> Result<(f64, f64), String> {
    let b = stack.pop().ok_or(format!("Not enough operands for '{}'", operator))?;
    let a = stack.pop().ok_or(format!("Not enough operands for '{}'", operator))?;
    Ok((a, b))
}

/// Calculate the result of an RPN expression
fn calculate_rpn(expression: &str) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in expression.split_whitespace() {
        match token {
            "+" => {
                let (a, b) = pop_two_operands(&mut stack, "+")?;
                stack.push(a + b);
            }
            "-" => {
                let (a, b) = pop_two_operands(&mut stack, "-")?;
                stack.push(a - b);
            }
            "*" => {
                let (a, b) = pop_two_operands(&mut stack, "*")?;
                stack.push(a * b);
            }
            "/" => {
                let (a, b) = pop_two_operands(&mut stack, "/")?;
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                stack.push(a / b);
            }
            "sin" => {
                let a = stack.pop().ok_or("Not enough operands for 'sin'".to_string())?;
                stack.push(a.sin());
            }
            "cos" => {
                let a = stack.pop().ok_or("Not enough operands for 'cos'".to_string())?;
                stack.push(a.cos());
            }
            "tan" => {
                let a = stack.pop().ok_or("Not enough operands for 'tan'".to_string())?;
                stack.push(a.tan());
            }
            "sqrt" => {
                let a = stack.pop().ok_or("Not enough operands for 'sqrt'".to_string())?;
                if a < 0.0 {
                    return Err("Square root of a negative number".to_string());
                }
                stack.push(a.sqrt());
            }
            number => {
                let parsed_number = f64::from_str(number).map_err(|_| format!("Invalid token: {}", number))?;
                stack.push(parsed_number);
            }
        }
    }

    if stack.len() != 1 {
        return Err("Invalid RPN expression".to_string());
    }

    Ok(stack[0])
}

fn main() {
    println!("Reverse Polish Notation (RPN) Calculator");
    println!("Supports basic operations (+, -, *, /), trig functions (sin, cos, tan), and floating-point numbers.");
    println!("Enter an RPN expression (e.g., '3.5 4.2 +', '90 sin'):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            println!("Goodbye!");
            break;
        }

        match calculate_rpn(trimmed_input) {
            Ok(result) => println!("Result: {:.6}", result),
            Err(error) => println!("Error: {}", error),
        }
    }
}

