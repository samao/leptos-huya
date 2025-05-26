use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let resp = expect_context::<ResponseOptions>();
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }
    view! {
        <Title text="404" />
        <h1>"404"</h1>
    }
}
