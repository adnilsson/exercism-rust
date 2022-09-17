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
    let fib: Option<Vec<u8>> = (1..=5).map(_fibonacci).collect();
    fib.unwrap()
}

fn _fibonacci(count: usize) -> Option<u8> {
    match count {
        0 => None,
        1 | 2 => Some(1),
        n => Some(_fibonacci(n - 2).unwrap() + _fibonacci(n - 1).unwrap()),
    }
}
