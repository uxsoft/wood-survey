use tabled::Tabled;

#[derive(Tabled)]
pub struct Material {
    pub name: String,
    pub quality: String,
    pub thickness: u32,
    pub width: u32,
    pub length: u32,
    pub price: f32,
}

impl Material {
    pub fn area(&self) -> u32 {
        self.width * self.length
    }

    pub fn norm_price(&self ) -> f32 {
        self.price / (self.area() as f32)
    }
}