use leptos::prelude::*;
use leptos_meta::Title;

#[server]
async fn get_user() -> Result<Vec<String>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use std::time::Duration;
        use tokio::time::sleep;
        sleep(Duration::from_secs(3)).await;
    }
    Ok(["张三", "李四", "王麻子"]
        .iter()
        .copied()
        .map(Into::into)
        .collect())
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Home" />
        <h1>"Welcome to Leptos!"</h1>
        <Await future=get_user() let:data>
            <Counter labels=data.clone().unwrap_or_default() />
        </Await>
    }
}

#[island]
fn Counter(labels: Vec<String>) -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <button class="text-blue-400 border" on:click=on_click>
            "Click Me: "
            {count}
        </button>
        <ul class="w-4/5 mx-auto mt-10 *:first:cursor-default *:last:select-none *:hover:bg-amber-950 *:bg-amber-700 *:text-white *:rounded-xl *:inline-block *:text-xl/relaxed *:px-3 flex justify-evenly">
            {labels.into_iter().map(|label| view! { <li>{label}</li> }).collect_view()}
        </ul>
    }
}
