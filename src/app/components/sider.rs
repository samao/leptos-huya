use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos_use::use_window_scroll;

#[component]
pub fn Sider() -> impl IntoView {
    #[allow(unused)]
    let (active, set_active) = signal(false);

    #[cfg(feature = "hydrate")]
    Effect::new(move || {
        let (_, offset_y) = use_window_scroll();

        Effect::new(move || {
            set_active.set(offset_y.get() > 350.0);
        });
    });

    view! {
        <Show when=move || active.get()>
            <nav class="flex fixed right-5 bottom-10 flex-col gap-y-1 justify-center items-center py-2 text-xs leading-5 text-gray-400 bg-white rounded-md w-[50px] drop-shadow-xs min-[1440px]:right-10 *:hover:text-[#f80]">
                <a class="flex flex-col gap-y-1 justify-center items-center group">
                    <div class="overflow-hidden absolute -top-1 -translate-y-full pointer-events-none h-[75px] w-18">
                        <img
                            src="/imgs/mm2.png"
                            class="bottom-0 duration-300 translate-y-0 group-hover:opacity-0 group-hover:translate-y-full size-full"
                            alt=""
                        />
                    </div>
                    <div class="flex absolute top-0 flex-col gap-y-1 items-center p-3 text-gray-400 whitespace-nowrap bg-white rounded-md opacity-0 duration-500 pointer-events-none group-hover:opacity-100 group-hover:pointer-events-auto w-[106px] drop-shadow-xs left-25 -z-10 after:w-40 after:bg-amber-300 after:absolute after:left-0 after:h-10 after:opacity-0 before:size-3 before:top-7 before:border-0 before:rotate-45 before:bg-white before:-translate-1/2 before:left-full before:border-l-0 before:border-b-0 before:absolute group-hover:-left-30">
                        <img class="size-20" src="/imgs/app-qrcode.png" alt="" />
                        <p>虎牙直播App <br />独家赛事随时享</p>
                        <button class="px-2 leading-6 rounded-3xl border border-current hover:text-white text-[#f80] hover:bg-[#f80]">
                            下载APP
                        </button>
                        <hr class="my-1 mt-2 w-full text-gray-300" />
                        <img width=74 height=55 src="/imgs/pc.png" alt="" />
                        <p>虎牙PC客户端 <br />畅想蓝光臻画质</p>
                        <button class="px-2 leading-6 rounded-3xl border border-current hover:text-white text-sky-600 hover:bg-sky-600">
                            下载PC端
                        </button>
                    </div>
                    <div class="relative size-[26px] *:absolute *:size-full">
                        <img src="/imgs/down.png" class="group-hover:hidden" />
                        <img src="/imgs/down-hover.png" class="hidden group-hover:block" />
                    </div>
                    下载
                </a>
                {[
                    (
                        "开播".to_string(),
                        "".to_string(),
                        "/imgs/cam.png".to_string(),
                        "/imgs/cam-hover.png".to_string(),
                    ),
                    (
                        "合作".to_string(),
                        "".to_string(),
                        "/imgs/cop.png".to_string(),
                        "/imgs/cop-hover.png".to_string(),
                    ),
                    (
                        "网游".to_string(),
                        "".to_string(),
                        "/imgs/netgame.png".to_string(),
                        "/imgs/netgame-hover.png".to_string(),
                    ),
                    (
                        "帮助".to_string(),
                        "".to_string(),
                        "/imgs/help.png".to_string(),
                        "/imgs/help-hover.png".to_string(),
                    ),
                    (
                        "举报".to_string(),
                        "".to_string(),
                        "/imgs/report.png".to_string(),
                        "/imgs/report-hover.png".to_string(),
                    ),
                    (
                        "客服".to_string(),
                        "".to_string(),
                        "/imgs/service.png".to_string(),
                        "/imgs/service-hover.png".to_string(),
                    ),
                ]
                    .into_iter()
                    .map(|(label, link, normal, hover)| {
                        view! { <Item label=label link=link normal=normal hover=hover /> }
                    })
                    .collect_view()}
                <a
                    class="flex flex-col justify-center items-center group"
                    on:click=move |e| {
                        e.prevent_default();
                        window().scroll_to_with_x_and_y(0.0, 0.0);
                    }
                >
                    <div class="relative size-[26px] *:absolute *:size-full">
                        <img src="/imgs/top.png" class="group-hover:hidden" />
                        <img src="/imgs/top-hover.png" class="hidden group-hover:block" />
                    </div>
                    顶部
                </a>
            </nav>
        </Show>
    }
}

#[component]
fn Item(label: String, link: String, normal: String, hover: String) -> impl IntoView {
    view! {
        <a class="flex flex-col justify-center items-center group" href=link>
            <div class="relative size-[26px] *:absolute *:size-full">
                <img src=normal class="group-hover:hidden" />
                <img src=hover class="hidden group-hover:block" />
            </div>
            {label}
        </a>
    }
}
