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
    let operands = get_n_operands(2, stack);
    let maybe_sum: Option<i32> = operands.into_iter().sum();

    maybe_sum.map_or(None, |value| Some(CalculatorInput::Value(value)))
}

fn subtract(stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    let operands = get_n_operands(2, stack);
    // zip_with is still nightly, so do zip + map instead
    operands[0]
        .zip(operands[1])
        .map(|(x, y)| CalculatorInput::Value(y - x))
}

fn multiply(stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    let operands = get_n_operands(2, stack);
    let maybe_prod: Option<i32> = operands.into_iter().product();

    maybe_prod.map_or(None, |value| Some(CalculatorInput::Value(value)))
}

fn divide(stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    let operands = get_n_operands(2, stack);
    // zip_with is still nightly, so do zip + map instead
    operands[0]
        .zip(operands[1])
        .map(|(x, y)| CalculatorInput::Value(y / x))
}

pub fn evaluate_one<'a>(
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

pub fn get_n_operands(n_operands: usize, stack: &mut Vec<CalculatorInput>) -> Vec<Option<i32>> {
    let mut ops = Vec::with_capacity(n_operands);
    for _ in 0..n_operands {
        ops.push(stack.pop().map(|ci| ci.value()).flatten());
    }
    ops
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_two_operands() {
        let mut stack = vec![CalculatorInput::Value(1), CalculatorInput::Value(2)];
        let ops = get_n_operands(2, &mut stack);
        assert!(stack.is_empty());
        assert_eq!(ops, vec![Some(2), Some(1)]);
    }

    #[test]
    fn test_get_two_operands_with_operator() {
        let mut stack = vec![CalculatorInput::Value(1), CalculatorInput::Add];
        let ops = get_n_operands(2, &mut stack);
        assert!(stack.is_empty());
        assert_eq!(ops, vec![None, Some(1)]);
    }
}
