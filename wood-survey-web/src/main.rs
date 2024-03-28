#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { 
            class: "link link-primary",
            to: Route::Home {}, 
            "Go to counter" 
        }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            class: "link link-primary",
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            class: "gap-1",
            h1 { "High-Five counter: {count}" }
            button { 
                class: "btn",
                onclick: move |_| count += 1, "Up high!" 
            }
            button { 
                class: "btn",
                onclick: move |_| count -= 1, "Down low!" 
            }
        }
    }
}
