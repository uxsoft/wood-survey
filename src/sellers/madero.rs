use anyhow::Result;
use scraper::ElementRef;
use scraper::Selector;
use super::material::*;
use super::WoodSeller;

pub trait HtmlExtensions<'a> {
    fn select_string(&self, selector: &str) -> String;
}

impl<'a> HtmlExtensions<'a> for ElementRef<'a> {
    fn select_string(&self, selector: &str) -> String {
        self.select(&Selector::parse(selector).unwrap()).collect::<Vec<_>>().first().unwrap().text().collect::<String>().trim().to_string()
    }
}


pub struct MaderoWoodSeller;

impl MaderoWoodSeller {
    pub fn new() -> Self {
        Self
    }
}

impl WoodSeller for MaderoWoodSeller {
    fn name(&self) -> String {
        "madero.eu".to_owned()
    }

    fn pages(&self) -> Result<Vec<String>> {
        Ok(vec![
            "https://madero.eu/sk/eshop/skarovka-priebezna".to_owned(),
            "https://madero.eu/sk/eshop/skarovka-cink".to_owned(),
        ])
    }

    fn fetch_page(&self, url: &String) -> Result<Vec<Material>> {
        let response = reqwest::blocking::get(url)?;
        let text = response.text()?;

        let document = scraper::Html::parse_document(&text);

        let materials = document
            .select(&Selector::parse(".product-grid .__content").unwrap())
            .map(|e| -> Option<Material> {
                let name = e.select_string(".__title a");
                
                let dim_split = name.split(" ").last().unwrap().split("_").collect::<Vec<_>>();
                let thickness = dim_split.get(0).unwrap().parse().unwrap_or(0);
                let width = dim_split.get(1).unwrap().parse().unwrap_or(0);
                let length = dim_split.get(2).unwrap().parse().unwrap_or(0);

                let price_str = e.select_string(".product-price--with-tax")
                    .split(" ")
                    .collect::<Vec<_>>()
                    .get(0)
                    .unwrap()
                    .to_string();
                
                let price = price_str
                    .trim_end_matches("€")
                    .replace(",", ".")
                    .parse()
                    .unwrap_or(0.) * 24.01;
                
                let quality = "A/B".to_string();

                let species = match name.split(" ").collect::<Vec<_>>().get(0).unwrap().trim() {
                    "bk" => WoodSpecies::Beech,
                    "db" => WoodSpecies::Oak,
                    s => WoodSpecies::Other(s.to_string())
                };
                
                return Some(Material::new(
                    "madero.eu".to_owned(), name, species, quality, thickness, width, length, price,
                ));
            })
            .flatten()
            .collect::<Vec<Material>>();

        return Ok(materials);
    }
}
