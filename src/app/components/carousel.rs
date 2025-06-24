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

    view! {
        <div
            on:mouseenter=move |_| set_paused.set(true)
            on:mouseleave=move |_| set_paused.set(false)
            class="overflow-hidden relative w-full h-full bg-gray-300 rounded-md group/carousel"
        >
            <Show when=|| true>
                <div
                    style=move || format!("--dist-x: -{}%", index.get() * 100)
                    class="flex h-full duration-500 translate-x-[var(--dist-x)] *:flex-none *:w-full *:h-full **:[img]:size-[100%]"
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
                <div class="flex absolute top-0 left-0 items-center w-full h-full">
                    <div class="hidden overflow-hidden justify-between items-center w-full *:size-[70px] *:bg-black/60 *:rounded-full *:hover:bg-[#f80] *:after:border-white *:after:block *:after:size-4 *:after:border-r-3 *:after:border-t-3 *:relative *:after:absolute *:after:top-1/2 *:after:left-3/4 *:after:-translate-1/2 *:after:rotate-225 *:-translate-x-1/2 group-hover/carousel:flex">
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
                            class="after:left-1/4! translate-x-1/2! after:rotate-45!"
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
                <ul class="flex absolute bottom-2.5 right-3 space-x-1 *:aria-[current]:w-8 *:aria-[current]:bg-[#ff9600] *:duration-100">
                    {(0..data.get().len())
                        .map(|id| {
                            view! {
                                <li
                                    aria_current=if id == index.get() { Some("page") } else { None }
                                    class="bg-white size-2 rounded-[8px]"
                                />
                            }
                        })
                        .collect_view()}
                </ul>
            </Show>
        </div>
    }
}
