use leptos::prelude::*;
use leptos_meta::Title;
use stylance::import_crate_style;

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let resp = expect_context::<ResponseOptions>();
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    import_crate_style!(css, "src/pages/not_found.module.scss");

    view! {
        <Title text="404" />
        <div class=css::found_body>
            <img
                class=css::img
                src="/imgs/error.2c65e354350f47124a3c7d2e4f20a9cf.png"
                alt="error"
            />
            <p title="UNKNOWN">页面走丢了</p>
        </div>
    }
}
