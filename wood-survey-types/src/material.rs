use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WoodSpecies {
    Oak,
    Beech,
    Spruce,
    Pine,
    Ash,
    Walnut,
    Birch,
    Poplar,
    Maple,
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
            WoodSpecies::Walnut => write!(f, "Walnut"),
            WoodSpecies::Birch => write!(f, "Birch"),
            WoodSpecies::Poplar => write!(f, "Poplar"),
            WoodSpecies::Maple => write!(f, "Maple"),
            WoodSpecies::Other(name) => write!(f, "Other ({})", name) 
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WoodType {
    Board,
    Plywood,
    OSB,
    MDF,
    Plank,
    Other(String)
}

impl std::fmt::Display for WoodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WoodType::Board => write!(f, "Board"),
            WoodType::Plywood => write!(f, "Plywood"),
            WoodType::OSB => write!(f, "OSB"),
            WoodType::MDF => write!(f, "MDF"),
            WoodType::Plank => write!(f, "Plank"),
            WoodType::Other(name) => write!(f, "Other ({})", name)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Material {
    pub seller: String,
    pub kind: WoodType,
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
    pub fn new(seller: String, kind: WoodType, name: String, species: WoodSpecies, quality: String, thickness: u32, width: u32, length: u32, price: f32) -> Material {
        let area = (width as f32) / 1000. * (length as f32) / 1000.;
        let norm_price = price / area;

        Material {
            seller,
            kind,
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
