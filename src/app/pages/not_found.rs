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
        <div class="flex flex-col justify-center items-center mx-auto h-[710px]">
            <img
                class="size-50"
                src="https://ssr-static.msstatic.com/h5/HuyaMainSSR/@public-online/imgs/error.2c65e354350f47124a3c7d2e4f20a9cf.png"
                alt="error"
            />
            <p title="UNKNOWN">页面走丢了</p>
        </div>
    }
}
