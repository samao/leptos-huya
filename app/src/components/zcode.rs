use leptos::prelude::*;

#[lazy]
async fn get_zcodes() -> Vec<(&'static str, &'static str)> {
    vec![
        ("+86", "中国"),
        ("+852", "香港特别行政区"),
        ("+853", "中国澳门特别行政区"),
        ("+886", "中国台湾地区"),
        ("+355", "阿尔巴尼亚"),
        ("+213", "阿尔及利亚"),
        ("+38", "俄罗斯"),
        ("+93", "阿富汗"),
    ]
}

#[component]
pub fn Zcode(set_active: WriteSignal<(String, String)>) -> impl IntoView {
    stylance::import_crate_style!(css, "src/components/zcode.module.scss");

    let opened = RwSignal::new(false);

    let close_pop = move || {
        opened.update(|data| *data = false);
    };

    let result = LocalResource::new(move || get_zcodes());
    let (code, set_code) = signal("+86");

    view! {
        <div
            class=css::zcode
            on:click=|e| {
                e.stop_propagation();
            }
        >
            <label class=css::head>
                <input type="checkbox" bind:checked=opened />
                <span>{move || code.get()}</span>
            </label>
            <ul class=css::pop>
                <Suspense fallback=|| {
                    "..."
                }>
                    {move || Suspend::new(async move {
                        let data = result.get().unwrap_or(vec![]);
                        data.into_iter()
                            .map(|(code, name)| {
                                view! {
                                    <li
                                        class=css::item
                                        on:click=move |_| {
                                            leptos::logging::log!("{} - {}", code, name);
                                            set_code.set(code);
                                            set_active
                                                .update(|data| *data = (code.to_owned(), name.to_owned()));
                                            close_pop();
                                        }
                                    >
                                        <span>{code}</span>
                                        <span>{name}</span>
                                    </li>
                                }
                            })
                            .collect_view()
                    })}
                </Suspense>
            </ul>
        </div>
    }
}
