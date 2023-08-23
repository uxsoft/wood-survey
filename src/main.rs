mod sellers;

use std::path::Path;
use sellers::*;
use tabled::Table;
use anyhow::Result;

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

fn crawl_all_separate() -> anyhow::Result<()> {
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

fn crawl_all_together() -> anyhow::Result<()> {
    let sellers: Vec<Box<dyn WoodSeller>> = vec![
        Box::new(PRonicWoodSeller::new()),
        Box::new(MaderoWoodSeller::new()),
    ];

    let master_materials: Vec<Material> = sellers
        .iter()
        .flat_map(|seller| {
            let materials = seller.fetch().unwrap();
            materials
        })
        .collect();

    to_csv(&master_materials, "master.csv")?;

    Ok(())
}

// #[tokio::main]
fn main() -> anyhow::Result<()> {
    // let seller = MaderoWoodSeller::new();
    // let materials = seller.fetch()?;
    // let table = Table::new(materials);
    // println!("{}", table);
    
    crawl_all_together()?;

    Ok(())
}
