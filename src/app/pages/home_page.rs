use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <Title text="Home" />
        <h1>"Welcome to Leptos!"</h1>
        <button class="text-blue-400 border" on:click=on_click>
            "Click Me: "
            {count}
        </button>
    }
}
