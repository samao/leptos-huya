use std::time::Duration;

use leptos::prelude::*;

stylance::import_crate_style!(css, "src/pages/game/banner.module.scss");

#[component]
pub fn Banner(imgs: Vec<String>) -> impl IntoView {
    let size = imgs.len();
    let (current, set_current) = signal(0);

    Effect::new(move || {
        if let Ok(handle) = set_interval_with_handle(
            move || {
                set_current.update(|data| {
                    *data = *data + 1;
                });
            },
            Duration::from_secs(2),
        ) {
            on_cleanup(move || {
                handle.clear();
            });
        }
    });

    view! {
        <div class=css::game_banner>
            <For
                each=move || imgs.clone().into_iter().enumerate()
                key=|(_, img_url)| img_url.clone()
                let((id, img_url))
            >
                <div
                    style=format!("--game-cover: url({})", img_url)
                    class=move || if current.get() % size == id { Some(css::show) } else { None }
                />
            </For>
            <ul>
                <For each=move || (0..size).into_iter() key=|id| *id let(id)>
                    <li
                        on:click=move |_| set_current.set(id)
                        class=move || {
                            if current.get() % size == id { Some(css::current) } else { None }
                        }
                    />
                </For>
            </ul>
        </div>
    }
}
