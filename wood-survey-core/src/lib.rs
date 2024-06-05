mod sellers;
mod currency;

use itertools::Itertools;
use sellers::*;

pub fn get_sellers() -> Vec<Box<dyn WoodSeller>> {
    vec![
        Box::new(PRonicWoodSeller::new()),
        Box::new(MaderoWoodSeller::new()),
        Box::new(DrevomaWoodSeller::new())
    ]
}

pub fn to_csv(items: &Vec<Material>, file_path: &str) -> anyhow::Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        // .delimiter(b';')
        .from_path(file_path)?;

    for item in items {
        wtr.serialize(item)?;
    }

    wtr.flush()?;
    Ok(())
}

pub async fn fetch_all() -> anyhow::Result<Vec<Material>> {
    let sellers = get_sellers();

    let futures = sellers.iter().map(|s| s.fetch());

    let results = futures::future::join_all(futures).await;

    let big_list = results.into_iter().flatten_ok().flatten().collect();

    Ok(big_list)
}
