pub struct Currency;

impl Currency {
    pub(crate) fn new() -> Self {
        Self
    }

    pub fn eur(&self) -> f32 {
        24.15
    }
}