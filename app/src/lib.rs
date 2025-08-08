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

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

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
    }
}

#[cfg(feature = "hydrate")]
#[macro_export]
macro_rules! json {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let obj = js_sys::Object::new();
        $(js_sys::Reflect::set(&obj, &$key.into(), &$value.into()).unwrap();)*
        obj
    }};
}

#[macro_export]
macro_rules! clsx {
    // 处理空输入的情况
    () => {
        ""
    };

    // 处理单个字符串的情况
    ($first:literal) => {
        $first
    };

    // 处理多个字符串的情况
    ($first:literal, $($rest:literal),+ $(,)?) => {
        concat!($first, $(" ", $rest),+)
    };

    // 处理大括号块语法
    {$($tokens:literal),* $(,)?} => {
        clsx!($($tokens),*)
    };
}

#[cfg(feature = "hydrate")]
pub fn has(lib_name: &str) -> bool {
    let window = js_sys::global();
    js_sys::Reflect::has(&window, &lib_name.into()).unwrap_or(false)
}

pub fn to_time_str(timestamp: i64) -> String {
    use chrono::{DateTime, Utc};
    let d = DateTime::from_timestamp(timestamp, 0).unwrap_or(Utc::now());
    format!("{}", d.format("%d日 %H:%M"))
}

pub fn to_time_str_format(timestamp: i64, format: &str) -> String {
    use chrono::{DateTime, Utc};
    let d = DateTime::from_timestamp(timestamp, 0).unwrap_or(Utc::now());
    format!("{}", d.format(format))
}
