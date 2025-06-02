use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};

mod pages;
use pages::{HomePage, InfoPage, NotFound};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    use leptos_router::components::A;
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (nav_links, ..) = signal(vec![
        ("首页", ""),
        ("信息", "info"),
        ("关于", "about"),
        ("招聘", "boss"),
        ("新闻", "news"),
    ]);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-tailwind.css" />

        // sets the document title
        <Title formatter=|title| format!("{} - Welcome to Leptos", title) />

        // content for this welcome page
        <Router>
            <header class="py-2">
                <nav class="flex justify-around px-20 mx-auto text-white gap-x-10
                *:aria-[current]:bg-sky-500 *:text-xl/[40px] *:whitespace-nowrap *:transition 
                *:duration-100 *:px-10 *:py-2 *:bg-red-400 *:rounded-md *:hover:bg-red-400 *:hover:font-bold">
                    <For
                        each=move || nav_links.get()
                        key=|(label, link)| format!("{}_{}", label, link)
                        let((label, link))
                    >
                        <A href=link>{label}</A>
                    </For>
                </nav>
            </header>
            <main>
                <FlatRoutes fallback=NotFound>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("info") view=InfoPage />
                </FlatRoutes>
            </main>
        </Router>
    }
}
