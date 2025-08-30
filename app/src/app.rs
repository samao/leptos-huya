use std::time::Duration;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

use models::User as ModelUser;

use crate::components::{Footer, Header, LeftNav, Login};
use crate::pages::{CatePage, Game, HomePage, LivePage, MatchPage, NotFound, VideoPage};

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
    show_login: bool,
    toast: String,
    user: Option<ModelUser>,
    user_data_loaded: bool,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let store = reactive_stores::Store::new(GlobalState::default());

    provide_context(store);

    Effect::new(move || {
        if store.toast().get().len() > 0
            && let Ok(handle) = set_timeout_with_handle(
                move || {
                    store.toast().set("".to_string());
                },
                Duration::from_secs(3),
            )
        {
            on_cleanup(move || {
                handle.clear();
            });
        }
    });

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
                        <Route path=path!("v") view=VideoPage />
                        <Route path=path!("g") view=Game />
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
                        <Route path=path!("c") view=CatePage />
                        <Route path=path!("l") view=LivePage />
                    </ParentRoute>
                    <Route path=path!("m") view=MatchPage />
                </Routes>
            </main>
        </Router>
        <Show when=move || store.show_login().get()>
            <Login />
        </Show>
        <Show when=move || store.toast().get().len().ne(&0)>
            <span class=css::toast>{store.toast().get_untracked()}</span>
        </Show>
    }
}
