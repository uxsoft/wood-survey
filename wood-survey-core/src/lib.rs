mod sellers;
mod currency;

use std::path::Path;
use sellers::*;

fn to_csv(items: &Vec<Material>, file_path: &str) -> anyhow::Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        // .delimiter(b';')
        .from_path(file_path)?;

    for item in items {
        wtr.serialize(item)?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn crawl_all_separate() -> anyhow::Result<()> {
    let sellers: Vec<(&str, Box<dyn WoodSeller>)> = vec![
        ("p-ronic.csv", Box::new(PRonicWoodSeller::new())),
        ("madero.csv", Box::new(MaderoWoodSeller::new())),
    ];

    for (seller_path, seller_module) in sellers {
        if Path::new(seller_path).exists() {
            let materials = seller_module.fetch()?;
            to_csv(&materials, seller_path)?;
        }
    }

    Ok(())
}

pub fn crawl_all_together() -> anyhow::Result<Vec<Material>> {
    let sellers: Vec<Box<dyn WoodSeller>> = vec![
        Box::new(PRonicWoodSeller::new()),
        Box::new(MaderoWoodSeller::new()),
        Box::new(DrevomaWoodSeller::new())
    ];

    let master_materials: Vec<Material> = sellers
        .iter()
        .flat_map(|seller| {
            let materials = seller.fetch().unwrap();
            materials
        })
        .collect();

    // to_csv(&master_materials, "master.csv")?;

    Ok(master_materials)
}
