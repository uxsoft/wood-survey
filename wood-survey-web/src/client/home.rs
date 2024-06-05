use leptos::*;
use leptos_meta::*;
use wood_survey_types::material::Material;
use crate::server::*;

#[component]
pub fn HomePage() -> impl IntoView {
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
        <Script src="/sorttable.js"/>
        <button class="btn" on:click=on_click>"Fetch"</button>
        // <div>
        //     {move || format!("{:?}", data.get())}
        // </div>
        <table class="sortable table table-xs">
            <thead>
                <th>"Seller"</th>
                <th>"Kind"</th>
                <th>"Name"</th>
                <th>"Species"</th>
                <th>"Quality"</th>
                <th>"Thickness"</th>
                <th>"Width"</th>
                <th>"Length"</th>
                <th>"Price"</th>
                <th>"Area (m2)"</th>
                <th>"Price (per m2)"</th>
            </thead>
            <tbody>
                <For 
                    each=move || data.get()
                    key=|m| m.name.clone()
                    children=move |m: Material| {
                        view! {
                            <tr>
                                <td>{m.seller.clone()}</td>
                                <td>{m.kind.to_string()}</td>
                                <td>{m.name.clone()}</td>
                                <td>{m.species.to_string()}</td>
                                <td>{m.quality.clone()}</td>
                                <td>{m.thickness.clone()}</td>
                                <td>{m.width.clone()}</td>
                                <td>{m.length.clone()}</td>
                                <td>{m.price.clone()}</td>
                                <td>{m.area_m2.clone()}</td>
                                <td>{m.price_per_m2.clone()}</td>
                            </tr>
                        }
                    }
                />
            </tbody>
        </table>
    }
}
