use leptos::{
    either::{Either, EitherOf3},
    html::Input,
    prelude::*,
};
use models::Post;

#[server]
async fn query_post(id: i32) -> Result<Post, ServerFnError> {
    match web_db::get_post(id) {
        Ok(Some(post)) => Ok(Post {
            id: post.id,
            title: post.title,
            body: post.body,
            published: post.published,
        }),
        Ok(None) => {
            #[cfg(feature = "ssr")]
            {
                use http::StatusCode;
                use leptos_axum::ResponseOptions;

                let response = expect_context::<ResponseOptions>();
                response.set_status(StatusCode::NOT_FOUND);
            }
            Err(ServerFnError::new("未找到".to_string()))
        }
        Err(er) => Err(ServerFnError::new(er.to_string())),
    }
}

#[component]
pub fn Game() -> impl IntoView {
    stylance::import_crate_style!(css, "src/pages/game.module.scss");

    let all_data = Resource::new(|| (), move |_| query_post(4));

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
                                    <div>
                                        <h2>{post.title}-- {post.published}</h2>
                                        {post.body}
                                    </div>
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
            <Transition set_pending fallback=|| view! { <p>post is loading!</p> }>
                {move || {
                    match result.get() {
                        Some(Ok(post)) => {
                            EitherOf3::A(
                                view! {
                                    <div>
                                        <p>{post.title}: published: {post.published}</p>
                                        <p>{post.body}</p>
                                    </div>
                                },
                            )
                        }
                        Some(Err(er)) => EitherOf3::B(view! { <p>{er.to_string()}</p> }),
                        None => EitherOf3::C(view! { <p>waiting for action dispatch</p> }),
                    }
                }}
            </Transition>
        </div>
    }
}
