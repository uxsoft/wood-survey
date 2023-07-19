mod sellers;
use sellers::*;
use tabled::Table;

fn main() -> anyhow::Result<()> {
    let materials = PRonicWoodSeller::fetch()?;
    let table = Table::new(materials).to_string();
    println!("{table}");
    
    Ok(())
}
