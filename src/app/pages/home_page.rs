use leptos::prelude::*;
use leptos_meta::Title;

use crate::app::components::Player;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Home" />
        <div class="w-[980px] min-[1440px]:w-[1220px] mx-auto">
            <div class="fixed left-0 top-0 h-[549px] min-[1440px]:h-[668px] w-full bg-[url(https://livewebbs2.msstatic.com/huya_1706588003_content.jpg)] bg-center bg-cover -z-10" />
            <div class="h-[467px] min-[1440px]:h-[586px] bg-green-400/40 flex gap-x-2 justify-between">
                <Player />
                <ul class="w-[129px] min-[1440px]:w-[161px]"></ul>
            </div>
        </div>
    }
}
