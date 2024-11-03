mod luhn;

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        luhn::is_valid(self.code.as_str())
    }
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }
}
