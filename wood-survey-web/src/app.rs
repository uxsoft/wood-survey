use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::server::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/wood-survey-web.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // let fetch_data_action = create_server_action::<FetchData>();
    let (data, set_data) = create_signal(vec![]);

    let on_click = move |_| {
        spawn_local(async move {
            let result = fetch_data().await;
            set_data.set(result.unwrap());
        })
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Fetch"</button>
        <div>
            {move || format!("{:?}", data.get())}
        </div>
    }
}
