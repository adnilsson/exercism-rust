/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let Some(mut digits) = parse_input(code) else {
        return false;
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
    checksum % 10 == 0
}

fn parse_input(input: &str) -> Option<Vec<u32>> {
    let filtered_input: Vec<_> = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();

    if filtered_input.len() < 2 {
        return None;
    }

    let digits: Vec<_> = filtered_input.into_iter().map(|c| c.to_digit(10)).collect();

    if digits.iter().any(Option::is_none) {
        return None;
    }

    Some(digits.into_iter().flatten().collect())
}
