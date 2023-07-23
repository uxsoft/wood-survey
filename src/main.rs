mod sellers;
use sellers::*;
use tabled::Table;

fn main() -> anyhow::Result<()> {
    let mut materials = PRonicWoodSeller::fetch()?;
    materials.sort_by(|a, b| a.price_per_m2.partial_cmp(&b.price_per_m2).unwrap());
    let table = Table::new(materials).to_string();
    println!("{table}");
    
    Ok(())
}
