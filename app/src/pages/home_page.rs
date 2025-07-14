use std::{sync::LazyLock, vec};

use leptos::prelude::*;
use leptos_meta::Title;

use crate::{
    clsx,
    components::{MediaType, Player, Sider},
};

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

static ROOM_RECOMMEND: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        "/imgs/home_recommend_live_web_1738919767.jpg",
        "/imgs/home_recommend_live_web_1748937095.jpg",
        "/imgs/home_recommend_live_web_1749051394.jpg",
        "/imgs/home_recommend_live_web_1745225789.jpg",
        "/imgs/home_recommend_live_web_1748920899.jpg",
        "/imgs/home_recommend_live_web_1749520873.jpg",
    ]
});

#[component]
pub fn HomePage() -> impl IntoView {
    let section = "mx-auto w-[980px] min-[1440px]:w-[1220px]";
    let hot_room_clsx = clsx! {
        "relative rounded-md opacity-80 hover:border-2 hover:shadow-none hover:opacity-100 hover:before:size-0",
        "hover:before:border-t-[8px] hover:before:border-b-[8px] hover:before:border-transparent hover:before:border-r-[12px]",
        "hover:before:border-r-[#ff9c00] hover:before:border-l-0 hover:before:absolute hover:before:-left-[12px]",
        "hover:before:top-1/2 hover:before:-translate-y-1/2 hover:border-[#ff9c00]"
    };
    let hot_rooms_clsx = clsx! {
        "flex flex-col justify-between rounded-r-md w-[129px] bg-black/40 min-[1440px]:w-[161px] *:rounded-md",
        "*:min-[1440px]:h-[91px] *:h-[71px] *:shadow-md *:shadow-black/60 *:place-content-center"
    };
    view! {
        <Title text="Home" />
        <div id="top" class=section>
            <div class="absolute top-0 left-0 w-full bg-center bg-cover h-[549px] min-[1440px]:h-[668px] bg-[url(/imgs/huya_1706588003_content.jpg)] -z-10" />
            <div class="flex relative gap-x-2 justify-evenly mt-2 h-[467px] min-[1440px]:h-[576px]">
                <Player media=MediaType::Hls(
                    "https://www.youtu.tv/stream/hls/master.m3u8".to_owned(),
                ) />
                <ul class=hot_rooms_clsx>
                    <For each=|| ROOM_RECOMMEND.clone().into_iter() key=|item| *item let(img_url)>
                        <li class=hot_room_clsx>
                            <img src=img_url alt="" class="rounded-md size-full" />
                        </li>
                    </For>
                </ul>
            </div>
            <LiveRooms />
            <HotNews />
        </div>
        <div class="relative py-4 mt-10 w-full bg-center bg-no-repeat bg-cover min-[1440px]:py-9 h-[397px] min-[1440px]:h-[482px] bg-[url(/imgs/recomBg.jpg)]">
            <div class=format!("h-full {}", section)>
                <HotCate />
            </div>
        </div>
        <div class=section>
            <CateRooms />
        </div>
        <Sider />
    }
}
