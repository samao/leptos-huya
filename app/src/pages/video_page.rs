use leptos::{either::Either, prelude::*};
use leptos_meta::Title;

#[server]
async fn get_all_vod() -> Result<Vec<models::VodSet>, ServerFnError> {
    use database::{establish_connection, vod_page::get_cate_vods};
    let conn = &mut establish_connection();
    get_cate_vods(conn).map_err(|er| ServerFnError::new(er.to_string()))
}

#[component]
pub fn VideoPage() -> impl IntoView {
    stylance::import_crate_style!(css, "src/pages/video_page.module.scss");
    let get_all_data = Resource::new(|| (), |_| async move { get_all_vod().await });
    view! {
        <Title text="UserPage" />
        <div class=css::video_page>
            <Suspense fallback=move || {
                "..."
            }>
                {Suspend::new(async move {
                    match get_all_data.await {
                        Ok(list) => {
                            Either::Right(
                                view! {
                                    <For
                                        each=move || list.clone().into_iter()
                                        key=|item| item.title.clone()
                                        let(cate)
                                    >
                                        <section>
                                            <div>
                                                <h1>
                                                    <img src=format!("/imgs/{}", cate.cover) />
                                                    {cate.title}
                                                </h1>
                                                <ul>
                                                    <For
                                                        each=move || cate.tags.clone().into_iter()
                                                        key=|item| item.clone()
                                                        let(item)
                                                    >
                                                        <li>{item}</li>
                                                    </For>
                                                </ul>
                                            </div>
                                            <div>
                                                <ul>
                                                    <For
                                                        each=move || cate.list.clone().into_iter()
                                                        key=|item| item.title.clone()
                                                        let(item)
                                                    >
                                                        <li>
                                                            <div>
                                                                <img src=format!("/imgs/{}", item.img_url) loading="lazy" />
                                                            </div>
                                                            <div>
                                                                <span>{item.title}</span>
                                                                <img src=format!("/imgs/{}", item.owner.avatar) />
                                                                <span>{item.owner.name}</span>
                                                                <span>{item.hots}</span>
                                                            </div>
                                                        </li>
                                                    </For>
                                                </ul>
                                                <ul>
                                                    <For
                                                        each=move || cate.rank.clone().into_iter()
                                                        key=|item| item.title.clone()
                                                        let(item)
                                                    >
                                                        <li>{item.title}</li>
                                                    </For>
                                                </ul>
                                            </div>
                                        </section>
                                    </For>
                                },
                            )
                        }
                        Err(er) => Either::Left(er.to_string()),
                    }
                })}
            </Suspense>
        </div>
    }
}
