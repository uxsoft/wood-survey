mod sellers;
mod currency;
use wood_survey_types::material::*;
use itertools::Itertools;
use sellers::*;

pub fn get_sellers() -> Vec<Box<dyn WoodSeller>> {
    vec![
        Box::new(PRonicWoodSeller::new()),
        Box::new(MaderoWoodSeller::new()),
        Box::new(DrevomaWoodSeller::new())
    ]
}

// pub fn to_csv(items: &Vec<Material>, file_path: &str) -> Result<(), String> {
//     let mut wtr = csv::WriterBuilder::new()
//         // .delimiter(b';')
//         .from_path(file_path)
//         .map_err(|e| e.to_string())?;

//     for item in items {
//         wtr.serialize(item)
//             .map_err(|e| e.to_string())?;
//     }

//     wtr.flush()
//         .map_err(|e| e.to_string())?;
//     Ok(())
// }

// pub async fn fetch_all() -> Result<Vec<Material>, String> {
//     let sellers = get_sellers();

//     let futures = sellers.iter().map(|s| s.fetch());

//     let results = futures::future::join_all(futures).await;

//     let big_list = results.into_iter().flatten_ok().flatten().collect();

//     Ok(big_list)
// }

pub async fn fetch_all_direct() -> Result<Vec<Material>, String> {
    let results = futures::future::join_all(vec![
        MaderoWoodSeller::new().fetch(),
        PRonicWoodSeller::new().fetch(),
        //DrevomaWoodSeller::new().fetch()
    ]).await;

    let big_list = results.into_iter().flatten_ok().flatten().collect();

    Ok(big_list)
}
