use std::{sync::LazyLock, vec};

use leptos::prelude::*;
use leptos_meta::Title;

use crate::app::components::Player;

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
    view! {
        <Title text="Home" />
        <div class="w-[980px] min-[1440px]:w-[1220px] mx-auto">
            <div class="fixed left-0 top-0 h-[549px] min-[1440px]:h-[668px] w-full bg-[url(https://livewebbs2.msstatic.com/huya_1706588003_content.jpg)] bg-center bg-cover -z-10" />
            <div class="h-[467px] min-[1440px]:h-[576px] mt-2 flex gap-x-2 justify-evenly relative">
                <Player />
                <ul class="w-[129px] bg-black/40 min-[1440px]:w-[161px] flex flex-col justify-between rounded-r-md *:rounded-md *:min-[1440px]:h-[91px] *:h-[71px] *:shadow-md *:shadow-black/60 *:place-content-center">
                    <For each=|| ROOM_RECOMMEND.clone().into_iter() key=|item| *item let(img_url)>
                        <li class="relative rounded-md hover:before:size-0
                        hover:before:border-t-[8px] hover:before:border-b-[8px] hover:before:border-transparent
                        hover:before:border-r-[12px] hover:before:border-r-[#ff9c00] hover:before:border-l-0
                        hover:before:absolute hover:before:-left-[12px] hover:before:top-1/2 hover:before:-translate-y-1/2
                        hover:border-2 hover:border-[#ff9c00] hover:shadow-none opacity-80 hover:opacity-100">
                            <img src=img_url alt="" class="size-full rounded-md" />
                        </li>
                    </For>
                </ul>
            </div>
        </div>
    }
}
