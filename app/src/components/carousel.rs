use leptos::logging::log;
use leptos::prelude::*;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Serialize, Clone)]
pub struct SlideItem {
    pub img_url: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[component]
pub fn Carousel(#[prop(default=Vec::new())] items: Vec<SlideItem>) -> impl IntoView {
    let (index, set_index) = signal(0);
    let (data, _) = signal(items);
    let total = move || data.get().len();
    let (paused, set_paused) = signal(false);

    Effect::new(move || {
        if let Ok(handle) = set_interval_with_handle(
            move || {
                if paused.get() {
                    return;
                }
                // log!("running {}", index.get_untracked());
                set_index.update(|index| {
                    *index = (*index + 1) % data.get_untracked().len();
                })
            },
            Duration::from_secs(3),
        ) {
            on_cleanup(move || {
                log!("ON CLEAN UP");
                handle.clear();
            });
        }
    });

    stylance::import_crate_style!(css, "src/components/carousel.module.scss");

    view! {
        <div
            on:mouseenter=move |_| set_paused.set(true)
            on:mouseleave=move |_| set_paused.set(false)
            class=css::carousel
        >
            <Show when=|| true>
                <div
                    style=move || format!("--dist-x: -{}%", index.get() * 100)
                    class=css::imgs
                >
                    <For
                        each=move || data.get().into_iter()
                        key=|item| item.to_owned().img_url
                        let(SlideItem { img_url, link })
                    >
                        <a href=link>
                            <img src=img_url alt="" />
                        </a>
                    </For>
                </div>
                <div class=css::btns>
                    <div class=css::bts_group>
                        <button on:click=move |_| {
                            set_index
                                .update(|current| {
                                    *current = if *current == 0 {
                                        total() - 1
                                    } else {
                                        *current - 1
                                    }
                                });
                        } />
                        <button
                            class=css::right
                            on:click=move |_| {
                                set_index
                                    .update(|current| {
                                        let total = total();
                                        *current = if *current == total - 1 {
                                            0
                                        } else {
                                            *current + 1
                                        };
                                    });
                            }
                        />
                    </div>
                </div>
                <ul class=css::dots>
                    {(0..data.get().len())
                        .map(|id| {
                            view! {
                                <li
                                    aria_current=if id == index.get() { Some("page") } else { None }
                                    class=css::dot
                                    on:click=move |_| {
                                        set_index.set(id);
                                    }
                                />
                            }
                        })
                        .collect_view()}
                </ul>
            </Show>
        </div>
    }
}
