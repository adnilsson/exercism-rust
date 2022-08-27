use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

// I'd argue that this function should return a Result<String> instead
pub fn value_to_color_string(value: u32) -> String {
    let maybe_color = ResistorColor::from_int(value).ok();

    match maybe_color {
        Some(color) => format!("{:?}", color),
        None => "value out of range".to_string(),
    }
}

/// Gather an enum iterator into a vector
fn enum_vec<T: Sequence>() -> Vec<T> {
    all::<_>().collect::<Vec<_>>()
}

pub fn colors() -> Vec<ResistorColor> {
    enum_vec::<ResistorColor>()
}
