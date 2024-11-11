/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    std::iter::zip(s1.chars(), s2.chars())
        .fold(0, |acc, pair| match pair {
            (a, b) if a != b => acc + 1,
            _ => acc,
        })
        .into()
}
