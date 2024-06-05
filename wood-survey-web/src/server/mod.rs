use leptos::*;

#[server]
pub async fn fetch_data() -> Result<usize, ServerFnError<String>> {
    let result = wood_survey_core::fetch_all_direct().await.unwrap();

    Ok(123)
}