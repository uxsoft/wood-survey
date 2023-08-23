use anyhow::Result;
use scraper::ElementRef;
use scraper::Selector;
use super::material::*;
use super::WoodSeller;

pub trait ElementRefExtensions<'a> {
    fn select_string(&self, selector: &str) -> String;
}

impl<'a> ElementRefExtensions<'a> for ElementRef<'a> {
    fn select_string(&self, selector: &str) -> String {
        self.select(&Selector::parse(selector).unwrap())
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_string()
    }
}

pub trait HtmlExtensions {
    fn select_attr(&self, selector: &str, attr_name: &str) -> String;
}

impl HtmlExtensions for scraper::Html {
    fn select_attr(&self, selector: &str, attr_name: &str) -> String {
        self.select(&Selector::parse(selector).unwrap())
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .value()
            .attr(attr_name)
            .unwrap()
            .to_string()
    }
}

pub struct MaderoWoodSeller;

impl MaderoWoodSeller {
    pub fn new() -> Self {
        Self
    }
}

fn parse_doc(document: scraper::Html, quality: String) -> Result<Vec<Material>> {
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

            let species = match name.split(" ").collect::<Vec<_>>().get(0).unwrap().trim() {
                "bk" => WoodSpecies::Beech,
                "db" => WoodSpecies::Oak,
                s => WoodSpecies::Other(s.to_string())
            };

            return Some(Material::new(
                "madero.eu".to_owned(), name, species, quality.clone(), thickness, width, length, price,
            ));
        })
        .flatten()
        .collect::<Vec<Material>>();

    return Ok(materials);
}

impl WoodSeller for MaderoWoodSeller {
    fn name(&self) -> String {
        "madero.eu".to_owned()
    }

    fn pages(&self) -> Result<Vec<String>> {
        Ok(vec![
            "https://madero.eu/cz/eshop/sparovka-prubezna".to_owned(),
            "https://madero.eu/cz/eshop/sparovka-cink".to_owned(),
        ])
    }

    fn fetch_page(&self, url: &str) -> Result<Vec<Material>> {
        let response = reqwest::blocking::get(url)?;
        let text = response.text()?;

        let document = scraper::Html::parse_document(&text);
        let page_count = document
            .select_attr("#pages-count-holder", "data-page-count")
            .parse()
            .unwrap_or(1);

        let quality = if url.ends_with("cink") { "A/B cink".to_string() } else { "A/B".to_string() };

        let mut master: Vec<Vec<Material>> = vec![];
        master.push(parse_doc(document, quality.clone())?);

        if page_count > 1 {
            for i in 2..page_count {
                let params = [("page", "2"), ("category", "sparovka-prubezna"), ("thickFrom", ""), ("thickTo", ""), ("widthFrom", ""), ("widthTo", ""), ("lengthFrom", ""), ("lengthTo", "")];

                let client = reqwest::blocking::Client::new();
                let res = client.post("https://madero.eu/cz/load-more")
                    .form(&params)
                    .send()
                    .unwrap();

                let text = res.text().unwrap();
                let doc = scraper::Html::parse_document(&text);

                master.push(parse_doc(doc, quality.clone())?);
            }
        }

        return Ok(master
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<Material>>());
    }
}
