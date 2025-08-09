use cfg_block::cfg_block;
use leptos::{html::Input, logging::log, prelude::*};
use leptos_meta::Title;
use leptos_router::hooks::query_signal;
use stylance::import_crate_style;

import_crate_style!(css, "src/pages/info_page.module.scss");

cfg_block! {
    #[cfg(feature="ssr")] {
        //use leptos::logging::log;
        use std::time::Duration;
        use tokio::time::sleep;
        use axum::http::request::Parts;
        //use axum::http::header::CONTENT_TYPE;
    }
}

#[server]
#[lazy]
async fn get_rows() -> Result<usize, ServerFnError> {
    let req_parts = use_context::<Parts>();
    if let Some(_req_parts) = req_parts {
        // log!("GET ROW: {:?}", req_parts.headers.get(CONTENT_TYPE));
    }
    sleep(Duration::from_secs(2)).await;
    Ok(888)
}

#[server]
async fn add_row(text: String) -> Result<usize, ServerFnError> {
    // log!("ADD ROW -> {}", text);
    sleep(Duration::from_secs(2)).await;

    if text == *"Error" {
        return Err(ServerFnError::new("Oh no! Could't add to server!"));
    }
    Ok(9528)
}

#[component]
pub fn InfoPage() -> impl IntoView {
    let action = ServerAction::<AddRow>::new();
    //action.version().get()
    let size = Resource::new(move || action.version().get(), |_| get_rows());
    let input_ref = NodeRef::<Input>::new();
    let size = move || {
        size.get().map_or("I Care!".into(), |val| {
            val.map_or("I HoHo".to_string(), |val| format!("i got {}", val))
        })
    };

    view! {
        <Title text="信息工程" />
        <div class=css::info_page>
            <button on:click:target=|evt| {
                log!("{}", evt.target().node_name());
            }>"GOD INFO PAGE"</button>
            <div class=css::inner>
                <input
                    class=css::input
                    type="text"
                    node_ref=input_ref
                    placeholder="type something here"
                />
                <button
                    class=css::submit
                    on:click=move |_| {
                        let text = input_ref.get().unwrap().value();
                        action.dispatch(text.into());
                    }
                >
                    submit
                </button>
                <Show when=move || action.pending().get()>
                    <div class=css::loading>8</div>
                </Show>
                <p>You submitted: {move || format!("{:?}", action.input().get())}</p>
                <p>You submitted: {move || format!("{:?}", action.value().get())}</p>
                <Transition fallback=|| "">
                    <p class=css::result>Total rows: {size}</p>
                </Transition>
            </div>
            <SimpleQueryCounter />
        </div>
    }
}

#[component]
pub fn SimpleQueryCounter() -> impl IntoView {
    let (count, set_count) = query_signal::<i32>("count");
    let clear = move |_| set_count.set(None);
    let decrement = move |_| set_count.set(Some(count.get().unwrap_or(0) - 1));
    let increment = move |_| set_count.set(Some(count.get().unwrap_or(0) + 1));

    view! {
        <div class=css::counter>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span class=css::c_value>"Value: " {move || count.get().unwrap_or(0)} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}
