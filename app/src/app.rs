//use leptos::portal::Portal;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path, StaticSegment,
};

use crate::components::{Footer, Header, LeftNav, Login};
use crate::pages::{Game, HomePage, InfoPage, MatchPage, NotFound, UserPage, VideoPage};

stylance::import_crate_style!(css, "src/app.module.scss");

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
                <HydrationScripts options islands=false />
                <link rel="stylesheet" href="/css/normalize.css" />
                <MetaTags />
            </head>
            <body class=css::root>
                <App />
            </body>
        </html>
    }
}

#[derive(Clone, Debug, Default, reactive_stores::Store)]
pub struct GlobalState {
    logined: bool,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let store = reactive_stores::Store::new(GlobalState::default());

    provide_context(store);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/huya-web.css" />

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
                                <div class=css::left_layout>
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
        <Show when=move || store.logined().get()>
            <Login />
        </Show>
    }
}
