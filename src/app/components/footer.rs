use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct FriendLink {
    href: Option<String>,
    label: String,
}

async fn get_friend_links() -> Vec<FriendLink> {
    vec![
        FriendLink {
            href: None,
            label: "腾讯游戏频道".into(),
        },
        FriendLink {
            href: None,
            label: "快看漫画".into(),
        },
        FriendLink {
            href: None,
            label: "爱拍".into(),
        },
        FriendLink {
            href: None,
            label: "搜狐56视频".into(),
        },
        FriendLink {
            href: None,
            label: "开放平台".into(),
        },
    ]
}

#[component]
pub fn Footer() -> impl IntoView {
    let friend_links = Resource::new(|| (), move |_| get_friend_links());

    view! {
        <div>
            <Suspense fallback=|| "loading friend links">
                <dl>
                    <dt>友情链接</dt>
                    <For
                        each=move || friend_links.get().unwrap().into_iter()
                        key=|link| link.label.clone()
                        let(link)
                    >
                        <dd>{link.label}</dd>
                    </For>
                </dl>
            </Suspense>
        </div>
    }
}
