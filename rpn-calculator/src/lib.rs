#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        match input {
            CalculatorInput::Value(n) => stack.push(*n),
            _ => {
                if stack.len() < 2 { return None }

                let b = stack.pop().unwrap_or(0);
                let a = stack.pop().unwrap_or(0);

                match input {
                    CalculatorInput::Add => stack.push(a + b),
                    CalculatorInput::Subtract => stack.push(a - b),
                    CalculatorInput::Multiply => stack.push(a * b),
                    CalculatorInput::Divide => stack.push(a / b),
                    CalculatorInput::Value(_) => ()
                }
            }
        }
    }

    if stack.len() != 1 { return None }

    stack.pop()
}
