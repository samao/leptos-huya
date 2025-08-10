use leptos::{
    either::{Either, EitherOf3},
    html::Input,
    prelude::*,
};
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

    let post_action = ServerAction::<QueryPost>::new();

    let (_, set_pending) = signal(false);

    let result = post_action.value();

    let node_ref = NodeRef::<Input>::new();

    view! {
        <div class=css::game>
            <h1>Auto resource</h1>
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
                                        {post.body}
                                    </p>
                                },
                            )
                        }
                        _ => Either::Left(view! { <p>发生了什么OOPS</p> }),
                    }
                })}
            </Suspense>
            <hr />
            <h1>Form component</h1>
            <form on:submit=move |evt| {
                evt.prevent_default();
                if let Some(node) = node_ref.get() {
                    if let Ok(id) = node.value().parse::<i32>() {
                        post_action.dispatch(QueryPost { id });
                    }
                }
            }>
                <input type="text" node_ref=node_ref />
                <button type="submit">GET POST</button>
            </form>
            <hr />
            <h1>Action Form</h1>
            <ActionForm action=post_action>
                <div>
                    <input type="text" name="id" />
                    <button type="submit">Action From GET</button>
                </div>
            </ActionForm>
            <hr />
            <Transition
                set_pending
                fallback= || view! { <p>post is loading!</p> }
            >
                {move || {
                    match result.get() {
                        Some(Ok(post)) => {
                            EitherOf3::A(view! {
                                <div>
                                    <p>{post.title}: published: {post.published}</p>
                                    <p>{post.body}</p>
                                </div>
                            })
                        },
                        Some(Err(er)) => {
                            EitherOf3::B(view! {
                                <p>{er.to_string()}</p>
                            })
                        },
                        None  => {
                            EitherOf3::C(view! {
                                <p>waiting for action dispatch</p>
                            })
                        },
                    }
                }}
            </Transition>
        </div>
    }
}
