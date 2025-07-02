use crate::app::components::Sider;
use leptos::prelude::*;

#[component]
pub fn Game() -> impl IntoView {
    view! {
        <div class="p-10 h-dvh">
            <Sider />
        </div>
    }
}
