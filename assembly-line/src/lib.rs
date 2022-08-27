const CARS_PER_HOUR: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let maximum_rate = CARS_PER_HOUR * speed as u32;
    let success_rate = match speed {
        1..=4 => 1.0,
        5..=8 => 0.90,
        9 | 10 => 0.77,
        _ => 0.0,
    };

    maximum_rate as f64 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let hourly_rate = production_rate_per_hour(speed);

    (hourly_rate / 60.0) as u32
}
