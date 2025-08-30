use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use models::SimCate as ModelSimCate;
use stylance::import_crate_style;

import_crate_style!(css, "src/pages/cate_page.module.scss");

#[server]
#[lazy]
async fn get_all_sim() -> Result<Vec<ModelSimCate>, ServerFnError> {
    use database::{establish_connection, sim_cate};
    let conn = &mut establish_connection();
    sim_cate::get_all(conn).map_err(|e| ServerFnError::new(e))
}

#[component]
pub fn CatePage() -> impl IntoView {
    let load_task = Resource::new(|| (), |_| async move { get_all_sim().await });
    view! {
        <Title text="信息工程" />
        <div class=css::cate_page>
            <Suspense>
                {move || Suspend::new(async move {
                    match load_task.await {
                        Ok(results) => {
                            Either::Right(
                                view! {
                                    <div class=css::right_box>
                                    <div class=css::cate_nav>
                                        全部分类
                                        <ul>
                                            <For
                                                each=|| vec!["全部", "网游竞技", "单机热游", "娱乐天地", "手游休闲"]
                                                key=|item| item.to_string()
                                                let(label)
                                            >
                                                <li>{label}</li>

                                            </For>
                                        </ul>
                                    </div>
                                    <ul class=css::cates>
                                   <For
                                       each=move || results.clone().into_iter()
                                       key=|item| item.src.clone()
                                       let(item)
                                   >
                                       <li>
                                           <img src=item.src.clone() loading="lazy" />
                                           <span>{item.name.clone()}</span>
                                       </li>
                                   </For>
                                   </ul>
                                   </div>
                                }
                            )
                        },
                        Err(e) => {
                            Either::Left(e.to_string())
                        }
                    }
                })}
            </Suspense>
        </div>
    }
}
