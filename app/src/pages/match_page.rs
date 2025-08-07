use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};
use stylance::import_crate_style;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Plant {
    name: String,
    weight: u32,
}

#[server]
#[lazy]
async fn get_all_plant() -> Result<Vec<Plant>, ServerFnError> {
    let plants = vec![
        Plant {
            name: "牵牛花".to_string(),
            weight: 12,
        },
        Plant {
            name: "凤仙花".to_string(),
            weight: 122,
        },
    ];
    Ok(plants)
    // Err(ServerFnError::new("Fing...".to_string()))
}

#[component]
pub fn MatchPage() -> impl IntoView {
    let get_plants = LocalResource::new(|| get_all_plant());
    import_crate_style!(css, "src/pages/match_page.module.scss");
    view! {
        <Suspense fallback=|| {
            "..."
        }>
            {move || match get_plants.get() {
                Some(Ok(plants)) => {
                    Either::Left(
                        view! {
                            <For
                                each=move || plants.clone().into_iter()
                                key=|plant| plant.name.clone()
                                let(plant)
                            >
                                <p class=css::line>{plant.name}: {plant.weight}</p>
                            </For>
                        },
                    )
                }
                _ => Either::Right("###"),
            }}
        </Suspense>
    }
}
