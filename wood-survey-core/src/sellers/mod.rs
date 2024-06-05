mod material;
mod p_ronic;
mod madero;
mod drevoma;
mod scraper_extensions;

use anyhow::Result;
use async_trait::async_trait;
pub use material::*;
pub use p_ronic::PRonicWoodSeller;
pub use madero::MaderoWoodSeller;
pub use drevoma::DrevomaWoodSeller;
use futures::future::join_all;

#[async_trait]
pub trait WoodSeller {
    fn name(&self) -> String;
    fn pages(&self) -> Result<Vec<String>>;
    async fn fetch_page(&self, url: &str) -> Result<Vec<Material>>;
    async fn fetch(&self) -> Result<Vec<Material>> {
        let pages = self.pages()?;
        let futures : Vec<_> = pages
            .iter()
            .map(|url| self.fetch_page(url))
            .collect();

        let results: Vec<Result<Vec<Material>>> = join_all(futures).await;

        let materials = results.into_iter().flat_map(|r| r.unwrap_or(vec![])).collect();

        Ok(materials)
    }
}


// madero.eu
// drevoma.sk
// https://www.unihobby.cz/preklizky
// https://www.drevo-kaplan.cz/preklizky
// https://www.demos-trade.cz/preklizky-truhlarske
// https://www.demos-trade.cz/sparovky/
// https://www.palubky-rezivo.eu/
