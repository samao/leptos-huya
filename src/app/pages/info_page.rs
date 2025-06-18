use cfg_block::cfg_block;
use leptos::{html::Input, prelude::*};
use leptos_meta::Title;
use leptos_router::hooks::query_signal;

cfg_block! {
    #[cfg(feature="ssr")] {
        use leptos::logging::log;
        use std::time::Duration;
        use tokio::time::sleep;
        use axum::http::request::Parts;
        use axum::http::header::CONTENT_TYPE;
    }
}

#[server]
async fn get_rows() -> Result<usize, ServerFnError> {
    let req_parts = use_context::<Parts>();
    if let Some(req_parts) = req_parts {
        log!("GET ROW: {:?}", req_parts.headers.get(CONTENT_TYPE));
    }
    sleep(Duration::from_secs(2)).await;
    Ok(888)
}

#[server]
async fn add_row(text: String) -> Result<usize, ServerFnError> {
    log!("ADD ROW -> {}", text);
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
        <div class="relative mx-auto w-3/4">
            GOD INFO PAGE <div>
                <input
                    class="px-1 rounded-md border border-gray-400 placeholder:text-xs"
                    type="text"
                    node_ref=input_ref
                    placeholder="type something here"
                />
                <button
                    class="px-2 ml-2 text-white rounded-md bg-[#f40]"
                    on:click=move |_| {
                        let text = input_ref.get().unwrap().value();
                        action.dispatch(text.into());
                    }
                >
                    submit
                </button>
                <Show when=move || action.pending().get()>
                    <div class="text-white bg-black rounded-full animate-spin size-20">8</div>
                </Show>
                <p>You submitted: {move || format!("{:?}", action.input().get())}</p>
                <p>You submitted: {move || format!("{:?}", action.value().get())}</p>
                <Transition fallback=|| "getting size">
                    <p class="text-2xl font-bold">Total rows: {size}</p>
                </Transition>
            </div> <SimpleQueryCounter />
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
        <div class="flex justify-center space-x-2 text-white *:[button]:rounded-md *:[button]:px-3 *:[button]:bg-amber-700">
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span class="mx-3 font-bold text-red-600">
                "Value: " {move || count.get().unwrap_or(0)} "!"
            </span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}
