use planet_macro_derive::Planet;

/// A Duration on planet Earth
#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;
    fn years_during(d: &Duration) -> f64 {
        let earth_years = d.seconds as f64 / Self::EARTH_YEAR_SECONDS;
        earth_years / Self::orbital_period_in_earth_years()
    }

    fn orbital_period_in_earth_years() -> f64;
}

#[derive(Planet)]
#[orbital_period = 0.2408467]
pub struct Mercury;

#[derive(Planet)]
#[orbital_period = 0.61519726]
pub struct Venus;

#[derive(Planet)]
#[orbital_period = 1]
pub struct Earth;

#[derive(Planet)]
#[orbital_period = 1.8808158]
pub struct Mars;

#[derive(Planet)]
#[orbital_period = 11.862615]
pub struct Jupiter;

#[derive(Planet)]
#[orbital_period = 29.447498]
pub struct Saturn;

#[derive(Planet)]
#[orbital_period = 84.016846]
pub struct Uranus;

#[derive(Planet)]
#[orbital_period = 164.79132]
pub struct Neptune;
