use std::fmt::Formatter;
use tabled::Tabled;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum WoodSpecies {
    Oak,
    Beech,
    Spruce,
    Pine,
    Ash,
    Other(String)
}

impl std::fmt::Display for WoodSpecies {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WoodSpecies::Oak => write!(f, "Oak"),
            WoodSpecies::Beech => write!(f, "Beech"),
            WoodSpecies::Spruce => write!(f, "Spruce"),
            WoodSpecies::Pine => write!(f, "Pine"),
            WoodSpecies::Ash => write!(f, "Ash"),
            WoodSpecies::Other(name) => write!(f, "Other ({})", name) 
        }
    }
}

#[derive(Tabled, Serialize, Clone)]
pub struct Material {
    pub seller: String,
    pub name: String,
    pub species: WoodSpecies,
    pub quality: String,
    pub thickness: u32,
    pub width: u32,
    pub length: u32,
    pub price: f32,
    pub area_m2: f32,
    pub price_per_m2: f32,
}

impl Material {
    pub fn new(seller: String, name: String, species: WoodSpecies, quality: String, thickness: u32, width: u32, length: u32, price: f32) -> Material {
        let area = (width as f32) / 1000. * (length as f32) / 1000.;
        let norm_price = price / area;

        Material {
            seller,
            name,
            species,
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
