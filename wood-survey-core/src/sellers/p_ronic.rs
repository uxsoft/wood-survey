use anyhow::Result;
use async_trait::async_trait;
use wood_survey_types::material::*;
use super::WoodSeller;

pub struct PRonicWoodSeller;

impl PRonicWoodSeller {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl WoodSeller for PRonicWoodSeller {
    fn name(&self) -> String {
        "p-ronic.cz".to_owned()
    }

    fn pages(&self) -> Result<Vec<String>> {
        Ok(vec![
            "https://www.p-ronic.com/sparovky/borovice.html".to_owned(),
            "https://www.p-ronic.com/sparovky/smrk.html".to_owned(),
            "https://www.p-ronic.com/sparovky/dub.html".to_owned(),
            "https://www.p-ronic.com/sparovky/buk.html".to_owned(),
            "https://www.p-ronic.com/sparovky/jasan.html".to_owned(),
        ])
    }

    async fn fetch_page(&self, url: &str) -> Result<Vec<Material>> {

        let response = reqwest::get(url).await?;
        let text = response.text().await?;

        let document = scraper::Html::parse_document(&text);

        let table_selector = scraper::Selector::parse("table > tbody > tr").unwrap();
        let cell_selector = scraper::Selector::parse("td").unwrap();

        let materials = document
            .select(&table_selector)
            .map(|e| -> Option<Material> {
                let cells = e.select(&cell_selector).collect::<Vec<_>>();

                let name = cells.get(0)?.text().collect::<String>().trim().to_string();
                
                let quality = cells
                    .get(1)?
                    .text()
                    .collect::<String>()
                    .trim()
                    .replace("-", "")
                    .to_string();
                
                let thickness = cells
                    .get(2)?
                    .text()
                    .collect::<String>()
                    .trim()
                    .parse()
                    .unwrap_or(0);

                let dimensions = cells
                    .get(3)?
                    .text()
                    .collect::<String>()
                    .split("x")
                    .map(|x| x.trim().to_owned())
                    .collect::<Vec<_>>();
                let width = dimensions.get(0)?.parse().unwrap_or(0);
                let length = dimensions.get(1)?.parse().unwrap_or(0);

                let price = cells
                    .get(5)?
                    .text()
                    .collect::<String>()
                    .trim()
                    .parse()
                    .unwrap_or(0.);

                let species = match name.split("-").last().unwrap().trim() {
                    "SMRK" => WoodSpecies::Spruce,
                    "BOR" => WoodSpecies::Pine,
                    "DUB" => WoodSpecies::Oak,
                    "JASAN" => WoodSpecies::Ash,
                    "BUK" => WoodSpecies::Beech,
                    s => WoodSpecies::Other(s.to_string())
                };
                
                return Some(Material::new(
                    "p-ronic.cz".to_owned(), WoodType::Board, name, species, quality, thickness, width, length, price,
                ));
            })
            .flatten()
            .collect::<Vec<Material>>();

        return Ok(materials);
    }
}
