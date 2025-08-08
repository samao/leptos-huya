use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{MediaType, Player, Sider};

mod home_recommend_room;
use home_recommend_room::LiveRooms;

mod hot_news;
use hot_news::HotNews;

mod cate_rooms;
use cate_rooms::CateRooms;

mod hot_cate;
use hot_cate::HotCate;

mod playbill;
use playbill::PlayBill;

#[lazy]
fn get_recommends() -> Vec<String> {
    vec![
        "/imgs/home_recommend_live_web_1738919767.jpg".to_string(),
        "/imgs/home_recommend_live_web_1748937095.jpg".to_string(),
        "/imgs/home_recommend_live_web_1749051394.jpg".to_string(),
        "/imgs/home_recommend_live_web_1745225789.jpg".to_string(),
        "/imgs/home_recommend_live_web_1748920899.jpg".to_string(),
        "/imgs/home_recommend_live_web_1749520873.jpg".to_string(),
    ]
}

#[component]
pub fn HomePage() -> impl IntoView {
    stylance::import_crate_style!(css, "src/pages/home_page.module.scss");
    let cates = Resource::new(|| (), |_| get_recommends());
    view! {
        <Title text="Home" />
        <div id="top" class=css::section>
            <div class=css::banner />
            <div class=css::top>
                <Player media=MediaType::Hls(
                    "https://www.youtu.tv/stream/hls/master.m3u8".to_owned(),
                ) />
                <ul class=css::hot_rooms_clsx>
                    <Suspense fallback=|| "Loading...">
                        <For
                            each=move || cates.get().unwrap_or(vec![]).into_iter()
                            key=|item| item.to_string()
                            let(img_url)
                        >
                            <li class=css::hot_room_clsx>
                                <img src=img_url class=css::cover />
                            </li>
                        </For>
                    </Suspense>
                </ul>
            </div>
            <LiveRooms />
            <HotNews />
        </div>
        <div class=css::cate>
            <div class=format!("{} {}", css::h_full, css::section)>
                <HotCate />
            </div>
        </div>
        <div class=css::section>
            <CateRooms />
        </div>
        <Sider />
    }
}
