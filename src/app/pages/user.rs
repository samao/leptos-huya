use leptos::ev::click;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_meta::Title;
use web_sys::Element;

fn add_dot(el: Element, amount: usize) {
    use leptos::wasm_bindgen::JsCast;
    let el = el.unchecked_into::<web_sys::HtmlElement>();

    log!("ADD DOT");

    let handle = el.clone().on(click, move |_| {
        el.set_inner_text(&format!("{}{}", el.inner_text(), ".".repeat(amount)))
    });
    on_cleanup(move || {
        log!("ADD DOT on_cleanup");
        drop(handle)
    });
}

#[component]
pub fn UserPage() -> impl IntoView {
    Effect::new(|| {
        log!("{:?}", serde_json::json!({"name": "aaa"}));

        log!("{:?}", crate::json! {"name" => "WWW"});
    });

    view! {
        <Title text="UserPage" />
        <div class="flex flex-col justify-center items-center mx-auto h-[710px]">
            <p use:add_dot=15 title="UNKNOWN">
                HAHAHA
            </p>
        </div>
    }
}
