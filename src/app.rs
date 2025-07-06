use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path, StaticSegment,
};

mod components;
use components::{Footer, Header, LeftNav};

mod pages;
use pages::{Game, HomePage, InfoPage, MatchPage, NotFound, UserPage, VideoPage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="viewport"
                    content="width=device-width, height=device-height, initial-scale=1"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
                <MetaTags />
            </head>
            <body class="select-none bg-[#f4f5f8] font-leptos!">
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
                <Routes fallback=NotFound>
                    <ParentRoute
                        path=StaticSegment("/")
                        view=|| {
                            view! {
                                <Outlet />
                                <Footer />
                            }
                        }
                    >
                        <Route path=path!("") view=HomePage />
                        <Route path=path!("video") view=UserPage />
                        <Route path=path!("game") view=Game />
                    </ParentRoute>
                    <ParentRoute
                        path=StaticSegment("/")
                        view=|| {
                            view! {
                                <div class="flex relative gap-x-3 h-[calc(100vh-60px)]">
                                    <LeftNav />
                                    <Outlet />
                                </div>
                            }
                        }
                    >
                        <Route path=path!("g") view=InfoPage />
                        <Route path=path!("l") view=VideoPage />
                    </ParentRoute>
                    <Route path=path!("m") view=MatchPage />
                </Routes>
            </main>
        </Router>
    }
}
