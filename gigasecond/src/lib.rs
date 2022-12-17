use time::Duration;
use time::PrimitiveDateTime as DateTime;

const GIGASECOND: time::Duration = Duration::seconds(i64::pow(10, 9));

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + GIGASECOND
}
