use tabled::Tabled;

#[derive(Tabled)]
pub struct Material {
    pub seller: String,
    pub name: String,
    pub quality: String,
    pub thickness: u32,
    pub width: u32,
    pub length: u32,
    pub price: f32,
    pub area_m2: f32,
    pub price_per_m2: f32,
}

impl Material {
    pub fn new(name: String, quality: String, thickness: u32, width: u32, length: u32, price: f32) -> Material {
        let area = (width as f32) / 1000. * (length as f32) / 1000.;
        let norm_price = price / area;

        Material {
            seller: "p-ronic.cz".to_owned(),
            name,
            quality,
            thickness,
            width,
            length,
            price,
            area_m2: area,
            price_per_m2: norm_price,
        }
    }
}
