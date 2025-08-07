use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos_use::use_window_scroll;

stylance::import_crate_style!(css, "src/components/sider.module.scss");

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
            <nav class=css::nav_container_clsx>
                <a class=format!("{} {} {}", css::group, css::common, css::head)>
                    <div class=css::toy>
                        <img
                            src="/imgs/mm2.png"
                            alt=""
                        />
                    </div>
                    <div class=css::download_clsx>
                        <img class=css::qrcode src="/imgs/app-qrcode.png" alt="" />
                        <p>虎牙直播App <br />独家赛事随时享</p>
                        <button class=css::download_btn>
                            下载APP
                        </button>
                        <hr class=css::hr />
                        <img width=74 height=55 src="/imgs/pc.png" alt="" />
                        <p>虎牙PC客户端 <br />畅想蓝光臻画质</p>
                        <button class=css::pc>
                            下载PC端
                        </button>
                    </div>
                    <div class=css::bottom_imgs>
                        <img src="/imgs/down.png" />
                        <img src="/imgs/down-hover.png"/>
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
                    class=format!("{} {} {}", css::group, css::common, css::to_top)
                    on:click=move |e| {
                        e.prevent_default();
                        window().scroll_to_with_x_and_y(0.0, 0.0);
                    }
                >
                    <div class=css::to_top_box>
                        <img src="/imgs/top.png" />
                        <img src="/imgs/top-hover.png" />
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
        <a class=format!("{} {} {}", css::group, css::common, css::common_item) href=link>
            <div class=css::common_box>
                <img src=normal />
                <img src=hover />
            </div>
            {label}
        </a>
    }
}
