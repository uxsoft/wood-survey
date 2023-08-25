use anyhow::Result;
use super::material::*;
use super::WoodSeller;
use super::scraper_extensions::*;
use regex::Regex;

pub struct DrevomaWoodSeller;

impl DrevomaWoodSeller {
    pub fn new() -> Self {
        Self
    }
}

impl WoodSeller for DrevomaWoodSeller {
    fn name(&self) -> String {
        "drevoma.sk".to_owned()
    }

    fn pages(&self) -> Result<Vec<String>> {
        Ok(vec![
            "https://www.drevoma.sk/kategoria/skarovky".to_owned(),
            "https://www.drevoma.sk/kategoria/skarovky?page=2".to_owned(),
            "https://www.drevoma.sk/kategoria/preglejky".to_owned(),
        ])
    }

    fn fetch_page(&self, url: &str) -> Result<Vec<Material>> {
        let currency = crate::currency::Currency::new();
        
        let response = reqwest::blocking::get(url)?;
        let text = response.text()?;
        
        let document = scraper::Html::parse_document(&text);

        let dimensions_regex = Regex::new(r"(?<t>[\d]+)x(?<w>[\d]+)x(?<l>[\d]+)(\s)?mm").unwrap();

        let materials = document
            .select(&scraper::Selector::parse("#products-list section").unwrap())
            .map(|e| -> Option<Material> {
                let name = e.select_string("a");
                let name_split = name.split(" ").collect::<Vec<_>>();


                let (_, quality) = name.split_once("mm").unwrap();
                let quality = quality.trim().to_string();

                let dimensions = dimensions_regex.captures(&name).unwrap();
                let thickness = dimensions["t"].parse().unwrap();
                let width = dimensions["w"].parse().unwrap();
                let length = dimensions["l"].parse().unwrap();

                let kind = match name_split.get(0).unwrap() {
                    &"Preglejka" => WoodType::Plywood,
                    &"Škárovka" => WoodType::Board,
                    &t => WoodType::Other(t.to_string())
                };

                let prices = e.select_string("div");
                let (dph, _) = prices.split_once("€").unwrap();
                let price : f32 = dph.replace(",", ".").trim().parse().unwrap();

                let species = match name_split.get(2).unwrap() {
                    &"Dub" => WoodSpecies::Oak,
                    &"Smrek" => WoodSpecies::Spruce,
                    &"Borovica" => WoodSpecies::Pine,
                    &"Orech" => WoodSpecies::Walnut,
                    &"Buk" => WoodSpecies::Beech,
                    &"Javor" => WoodSpecies::Maple,
                    &"Breza" | _ if name.contains("Breza") => WoodSpecies::Birch,
                    &"Jaseň" | _ if name.contains("Jaseň") => WoodSpecies::Ash,
                    &"Topoľ" | _ if name.contains("Topoľ") => WoodSpecies::Poplar,
                    s => WoodSpecies::Other(s.to_string())
                };

                return Some(Material::new(
                    "drevoma.sk".to_owned(), kind, name, species, quality, thickness, width, length, price  * currency.eur(),
                ));
            })
            .flatten()
            .collect::<Vec<Material>>();

        return Ok(materials);
    }
}
