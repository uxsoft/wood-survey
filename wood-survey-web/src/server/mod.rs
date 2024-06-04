use leptos::*;

#[server]
pub async fn fetch_data() -> Result<usize, ServerFnError<String>> {
    use wood_survey_core::crawl_all_separate;

    let result = wood_survey_core::crawl_all_together().unwrap();

    Ok(123)
}