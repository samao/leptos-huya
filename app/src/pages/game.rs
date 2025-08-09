use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

cfg_block::cfg_block! {
    #[cfg(feature="ssr")] {
        async fn get_post(id: i32) -> Result<web_db::models::Post, String> {
            use web_db::get_post;
            match get_post(id) {
                Some(post) => Ok(post),
                _ => Err("查找错误".to_string()),
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Post {
    title: String,
    body: String,
    published: bool,
}

#[server]
async fn query_post(id: i32) -> Result<Post, ServerFnError> {
    match get_post(id).await {
        Ok(post) => Ok(Post {
            title: post.title,
            body: post.body,
            published: post.published,
        }),
        Err(msg) => Err(ServerFnError::new(msg)),
    }
}

#[component]
pub fn Game() -> impl IntoView {
    stylance::import_crate_style!(css, "src/pages/game.module.scss");

    let all_data = Resource::new(|| (), move |_| query_post(1));
    view! {
        <div class=css::game>
            <Suspense fallback=|| {
                "loading..."
            }>
                {move || Suspend::new(async move {
                    match all_data.get() {
                        Some(Ok(post)) => {
                            Either::Right(
                                view! {
                                    <p>
                                        <b>{post.title}-- {post.published}</b>
                                        <br />
                                        {post.body}
                                    </p>
                                },
                            )
                        }
                        _ => Either::Left(view! { <p>发生了什么OOPS</p> }),
                    }
                })}
            </Suspense>
        </div>
    }
}
