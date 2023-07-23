mod material;
mod p_ronic;

use anyhow::Result;
pub use material::*;
pub use p_ronic::*;

pub trait WoodSeller {
    fn name() -> String;
    fn pages() -> Result<Vec<String>>;
    fn fetch_page(url: &String) -> Result<Vec<Material>>;
    fn fetch() -> Result<Vec<Material>> {
        let materials = Self::pages()?
            .iter()
            .flat_map(|url| Self::fetch_page(url).ok())
            .flatten()
            .collect::<Vec<Material>>();

        Ok(materials)
    }
}



//https://www.unihobby.cz/preklizky
//https://www.drevo-kaplan.cz/preklizky
//https://www.demos-trade.cz/preklizky-truhlarske
//https://www.demos-trade.cz/sparovky/