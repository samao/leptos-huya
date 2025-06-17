use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};

mod components;
use components::Header;

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
            <body class="bg-[#f4f5f8]">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-tailwind.css" />

        // sets the document title
        <Title formatter=|title| format!("{} - Welcome to Leptos", title) />

        // content for this welcome page
        <Router>
            <Header />
            <main>
                <FlatRoutes fallback=NotFound>
                    <Route path=StaticSegment("/") view=HomePage />
                    <Route path=StaticSegment("/g") view=InfoPage />
                </FlatRoutes>
            </main>
        </Router>
    }
}
