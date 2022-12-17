#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    fn value(&self) -> Option<i32> {
        if let &Self::Value(val) = self {
            Some(val)
        } else {
            None
        }
    }
}

fn add(stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    do_binary_op(|x, y| x + y, stack)
}

fn subtract(stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    do_binary_op(|x, y| x - y, stack)
}

fn multiply(stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    do_binary_op(|x, y| x * y, stack)
}

fn divide(stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    do_binary_op(|x, y| x / y, stack)
}

pub fn evaluate_one(
    input: &CalculatorInput,
    stack: &mut Vec<CalculatorInput>,
) -> Option<CalculatorInput> {
    match input {
        &CalculatorInput::Value(val) => Some(CalculatorInput::Value(val)),
        CalculatorInput::Add => add(stack),
        CalculatorInput::Subtract => subtract(stack),
        CalculatorInput::Multiply => multiply(stack),
        CalculatorInput::Divide => divide(stack),
    }
}

pub fn do_binary_op(
    operation: fn(i32, i32) -> i32,
    stack: &mut Vec<CalculatorInput>,
) -> Option<CalculatorInput> {
    let mut pop_operand = || stack.pop().map(|ci| ci.value()).flatten();
    pop_operand()
        .zip(pop_operand())
        .map(|(op2, op1)| operation(op1, op2))
        .map(|op_result| CalculatorInput::Value(op_result))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<CalculatorInput> = Vec::new();

    for input in inputs {
        if let Some(evaluated) = evaluate_one(input, &mut stack) {
            println!("Evaluated: {:?}; Stack: {:?}", evaluated, stack);
            stack.push(evaluated);
        } else {
            return None;
        }
    }

    let final_ci = stack.pop();
    if !stack.is_empty() {
        return None;
    }

    match final_ci {
        Some(CalculatorInput::Value(val)) => Some(val),
        _ => None,
    }
}
