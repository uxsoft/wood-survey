use anyhow::Result;
use super::material::*;
use super::WoodSeller;

pub struct PRonicWoodSeller;
impl WoodSeller for PRonicWoodSeller {
    fn name() -> String {
        "p-ronic.cz".to_owned()
    }

    fn pages() -> Result<Vec<String>> {
        Ok(vec![
            "https://www.p-ronic.com/sparovky/borovice.html".to_owned(),
            "https://www.p-ronic.com/sparovky/smrk.html".to_owned(),
            "https://www.p-ronic.com/sparovky/dub.html".to_owned(),
            "https://www.p-ronic.com/sparovky/buk.html".to_owned(),
            "https://www.p-ronic.com/sparovky/jasan.html".to_owned(),
        ])
    }

    fn fetch_page(url: &String) -> Result<Vec<Material>> {
        let response = reqwest::blocking::get(url)?;
        let text = response.text()?;

        let document = scraper::Html::parse_document(&text);

        let table_selector = scraper::Selector::parse("table > tbody > tr").unwrap();
        let cell_selector = scraper::Selector::parse("td").unwrap();

        let materials = document
            .select(&table_selector)
            .map(|e| -> Option<Material> {
                let cells = e.select(&cell_selector).collect::<Vec<_>>();

                let name = cells.get(0)?.text().collect::<String>().trim().to_string();
                let quality = cells.get(1)?.text().collect::<String>().trim().to_string();
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

                return Some(Material::new(
                    name, quality, thickness, width, length, price,
                ));
            })
            .flatten()
            .collect::<Vec<Material>>();

        return Ok(materials);
    }


}
