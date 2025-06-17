use std::{sync::LazyLock, vec};

use leptos::prelude::*;
use leptos_meta::Title;

use crate::app::components::{Footer, MediaType, Player};

mod home_recommend_room;
use home_recommend_room::LiveRooms;

static ROOM_RECOMMEND: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        "https://livewebbs2.msstatic.com/home_recommend_live_web_1738919767.jpg",
        "https://livewebbs2.msstatic.com/home_recommend_live_web_1748937095.jpg",
        "https://livewebbs2.msstatic.com/home_recommend_live_web_1749051394.jpg",
        "https://livewebbs2.msstatic.com/home_recommend_live_web_1745225789.jpg",
        "https://livewebbs2.msstatic.com/home_recommend_live_web_1748920899.jpg",
        "https://livewebbs2.msstatic.com/home_recommend_live_web_1749520873.jpg",
    ]
});

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <Title text="Home" />
        <div class="mx-auto w-[980px] min-[1440px]:w-[1220px]">
            <div class="absolute top-0 left-0 w-full bg-center bg-cover h-[549px] min-[1440px]:h-[668px] bg-[url(https://livewebbs2.msstatic.com/huya_1706588003_content.jpg)] -z-10" />
            <div class="flex relative gap-x-2 justify-evenly mt-2 h-[467px] min-[1440px]:h-[576px]">
                <Player media=MediaType::Hls(
                    "https://www.youtu.tv/stream/hls/master.m3u8".to_owned(),
                ) />
                <ul class="flex flex-col justify-between rounded-r-md w-[129px] bg-black/40 min-[1440px]:w-[161px] *:rounded-md *:min-[1440px]:h-[91px] *:h-[71px] *:shadow-md *:shadow-black/60 *:place-content-center">
                    <For each=|| ROOM_RECOMMEND.clone().into_iter() key=|item| *item let(img_url)>
                        <li class="relative rounded-md opacity-80 hover:border-2 hover:shadow-none hover:opacity-100 hover:before:size-0 hover:before:border-t-[8px] hover:before:border-b-[8px] hover:before:border-transparent hover:before:border-r-[12px] hover:before:border-r-[#ff9c00] hover:before:border-l-0 hover:before:absolute hover:before:-left-[12px] hover:before:top-1/2 hover:before:-translate-y-1/2 hover:border-[#ff9c00]">
                            <img src=img_url alt="" class="rounded-md size-full" />
                        </li>
                    </For>
                </ul>
            </div>
            <LiveRooms />
            <button on:click=move |_| *set_count.write() += 1>"Click me: " {count}</button>
            <p>"Double count: " {move || count.get() * 2}</p>
        </div>
        <Footer />
    }
}
