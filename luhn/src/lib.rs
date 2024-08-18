/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digits = match parse_input(code) {
        Some(digits) => digits,
        None => return false,
    };

    let multipliers = [1, 2].into_iter().cycle();
    digits = digits
        .into_iter()
        .rev()
        .zip(multipliers)
        .map(|(digit, multiplier)| digit * multiplier)
        .map(|prod_digit| (prod_digit / 10) + (prod_digit % 10))
        .collect();

    let checksum: u32 = digits.iter().sum();
    match checksum % 10 {
        0 => true,
        _ => false,
    }
}

fn parse_input(input: &str) -> Option<Vec<u32>> {
    let filtered_input: Vec<_> = input
        .chars()
        .into_iter()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

    if filtered_input.len() < 2 {
        return None;
    }

    let digits: Vec<_> = filtered_input
        .into_iter()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| c.to_digit(10))
        .collect();

    if digits.iter().any(|d| d.is_none()) {
        return None;
    }

    Some(digits.into_iter().flatten().collect())
}
