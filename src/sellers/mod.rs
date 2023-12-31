mod material;
mod p_ronic;
mod madero;
mod drevoma;
mod scraper_extensions;

use anyhow::Result;
pub use material::*;
pub use p_ronic::PRonicWoodSeller;
pub use madero::MaderoWoodSeller;
pub use drevoma::DrevomaWoodSeller;
pub use super::currency::*;
pub trait WoodSeller {
    fn name(&self) -> String;
    fn pages(&self) -> Result<Vec<String>>;
    fn fetch_page(&self, url: &str) -> Result<Vec<Material>>;
    fn fetch(&self) -> Result<Vec<Material>> {
        let materials: Vec<Material> = self.pages()?
            .iter()
            .flat_map(|url| {
                    match self.fetch_page(url) {
                        Ok(r) => r,
                        Err(err) => { eprintln!("Error fetching {} {}", url, err); return vec![] }
                    }
            })
            .collect();

        Ok(materials)
    }
}


// madero.eu
// drevoma.sk
// https://www.unihobby.cz/preklizky
// https://www.drevo-kaplan.cz/preklizky
// https://www.demos-trade.cz/preklizky-truhlarske
// https://www.demos-trade.cz/sparovky/