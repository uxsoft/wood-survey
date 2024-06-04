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
    let fetch_data_action = create_server_action::<FetchData>();
    let data = create_resource(move || fetch_data_action.version().get(), |_| fetch_data());

    let (count, set_count) = create_signal(0);
    let on_click = move |_| {
        fetch_data_action.dispatch(FetchData {});
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Fetch"</button>
        <div>
            {move || format!("{:?}", fetch_data_action.input().get())}
        </div>
    }
}
