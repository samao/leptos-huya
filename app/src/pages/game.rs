use leptos::prelude::*;

#[component]
pub fn Game() -> impl IntoView {
    stylance::import_crate_style!(css, "src/pages/game.module.scss");

    view! {
        <div class=css::game>
        </div>
    }
}
