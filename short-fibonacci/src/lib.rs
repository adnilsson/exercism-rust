/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let fib: Result<Vec<u8>, String> = (1..=5).map(_fibonacci).collect();
    fib.unwrap()
}

fn _fibonacci(count: usize) -> Result<u8, String> {
    match count {
        0 => Ok(0),
        1 | 2 => Ok(1),
        n => {
            let fib1 = _fibonacci(n - 2).unwrap();
            let fib2 = _fibonacci(n - 1).unwrap();

            match fib1.checked_add(fib2) {
                Some(checked_add) => Ok(checked_add),
                None => Err(format!("The {}:th fibonacci number can't fit into a u8", n)),
            }
        }
    }
}
