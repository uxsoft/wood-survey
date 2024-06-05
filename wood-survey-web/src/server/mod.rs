use leptos::*;
use wood_survey_types::material::*;


#[server]
pub async fn fetch_data() -> Result<Vec<Material>, ServerFnError<String>> {
    let result = wood_survey_core::fetch_all_direct().await.unwrap();

    Ok(result)
}