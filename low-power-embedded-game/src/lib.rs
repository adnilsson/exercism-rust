/// This function can overflow if the dividend is `-2^(n-1)` and the divisor is `-1`.
/// Overflowing causes the function to panic, regardless of build mode.
pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend.checked_div(divisor).unwrap();
    let remainder = dividend.checked_rem(divisor).unwrap();

    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

/// This function is defined for `Positions` `p` where `|p.0| + |p.1| < 2^(n-1)`.
/// Otherwise, the function panics, regardless of build mode.
/// ```should_panic
/// let x = (-2_i16).pow(15)+1; // x.abs() will be the largest positive value representable by an i16
/// let p = low_power_embedded_game::Position(x, 1);
/// p.manhattan(); // The final additition overflows
/// ```
pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let manhattan_x = self.0.checked_abs().unwrap();
        let manhattan_y = self.1.checked_abs().unwrap();

        manhattan_x.checked_add(manhattan_y).unwrap()
    }
}
