use leptos::*;
use leptos_meta::*;
use wood_survey_types::material::Material;
use crate::server::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (data, set_data) = create_signal(vec![]);

    create_effect(move |_|{
        spawn_local(async move {
            let result = fetch_data().await;
            set_data.set(result.unwrap());
        });
    });

    view! {
        <Script src="https://cdn.jsdelivr.net/npm/table-sort-js/table-sort.min.js"/>
        
        <table class="table-sort table table-xs">
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
