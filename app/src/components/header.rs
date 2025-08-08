use leptos::prelude::*;
use leptos_router::{
    components::{Form, A},
    hooks::use_url,
};
#[cfg(feature = "hydrate")]
use leptos_use::use_window_scroll;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, sync::LazyLock};

stylance::import_crate_style!(css, "src/components/header.module.scss");

static CATEGROY: LazyLock<HashMap<&str, Vec<&str>>> = LazyLock::new(|| {
    HashMap::from([
        (
            "热门分类",
            vec![
                "英雄联盟",
                "王者荣耀",
                "CS2",
                "户外",
                "体育",
                "一起看",
                "穿越火线",
                "天天吃鸡",
                "无畏契约",
                "星秀",
                "原始",
                "二次元",
            ],
        ),
        (
            "玩家推荐",
            vec![
                "三国杀",
                "欢乐斗地主",
                "JJ斗地主",
                "坦克世界",
                "我的世界",
                "魔兽世界",
                "守望先锋",
                "欢乐麻将",
                "问道",
                "梦三国",
                "天天狼人",
                "命运方舟",
            ],
        ),
    ])
});

static DOWNLOADS: LazyLock<Vec<DownloadItem>> = LazyLock::new(|| {
    vec![
        (
            "虎牙APP",
            "独家赛事随时享",
            "/imgs/h-code.png",
            "扫码下载",
            108,
            108,
        )
            .into(),
        (
            "虎牙PC客户端",
            "畅想蓝光臻画质",
            "/imgs/h-pc2.png",
            "点击下载",
            108,
            108,
        )
            .into(),
        (
            "虎牙TV电视端",
            "巨幕蓝光沉浸体验",
            "/imgs/h-TV.png",
            "点击下载",
            108,
            1088,
        )
            .into(),
    ]
});

static TASKS: LazyLock<Vec<Task<&str>>> = LazyLock::new(|| {
    vec![
        Task {
            img_url: "/imgs/wz_dianquan_s.png",
            cost_value: "60点券",
            value: 6000,
        },
        Task {
            img_url: "/imgs/cfm_dianquan_s.png",
            cost_value: "60点券",
            value: 6000,
        },
        Task {
            img_url: "/imgs/lolm_dianquan_s.png",
            cost_value: "500CF点",
            value: 5000,
        },
        Task {
            img_url: "/imgs/hy_huliang_s.png",
            cost_value: "虎粮*2",
            value: 200,
        },
        Task {
            img_url: "/imgs/hy_huliang_s.png",
            cost_value: "虎粮*10",
            value: 1000,
        },
    ]
});

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DownloadItem<'a> {
    title: &'a str,
    description: &'a str,
    img_url: &'a str,
    type_label: &'a str,
    size: Size,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Size(u32, u32);

type Sequment<'a> = (&'a str, &'a str, &'a str, &'a str, u32, u32);

impl<'a> From<Sequment<'a>> for DownloadItem<'a> {
    fn from((title, description, img_url, type_label, width, height): Sequment<'a>) -> Self {
        DownloadItem {
            title,
            description,
            img_url,
            type_label,
            size: Size(width, height),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task<T: ToString + Sized + 'static> {
    img_url: T,
    cost_value: T,
    value: u32,
}

async fn get_adv_info() -> String {
    "/imgs/huya_1716264051_content.gif".to_owned()
}

#[component]
fn Ad() -> impl IntoView {
    let show_ad = RwSignal::new(true);
    let result = Resource::new(|| (), |_| get_adv_info());
    let result = move || result.get().unwrap_or_else(|| "".to_owned());

    view! {
        <Suspense fallback=move || "loading">
            <Show when=move || show_ad.get() fallback=|| "">
                <div class=css::adv>
                    <img src=result width="274" height="65" />
                    <span on:click=move |_| show_ad.set(false)>x</span>
                </div>
            </Show>
        </Suspense>
    }
}

struct HeaderIco<'a> {
    name_label: &'a str,
    normal: &'a str,
    hover: &'a str,
}

impl<'a> Display for HeaderIco<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "--{}-icon: url({}); --{}-hover: url({})",
            self.name_label, self.normal, self.name_label, self.hover
        )
    }
}
#[component]
pub fn Header() -> impl IntoView {
    let matched = use_url();
    let stickied = RwSignal::new(false);
    let node_ref = NodeRef::<leptos::html::Header>::new();

    #[cfg(feature = "hydrate")]
    Effect::new(move || {
        if matched.get().path() == "/" {
            let (_, y) = use_window_scroll();

            Effect::new(move || {
                stickied.set(y.get() > 65.0);
            });
        }
    });

    view! {
        <header
            can-sticky=move || stickied.get()
            node_ref=node_ref
            style=move || match (matched.get().path(), stickied.get()) {
                ("/", false) => {
                    format!(
                        "{}; --triangle-opacity: {}; --right-header: {}; --right-header-hover: {}; --logo-url: url({}); --search-icon: {}; --search-icon-hover: {}; --search-bg: {}; --search-bg-hover: {}; --search-border: {}; {}; {}; {}",
                        HeaderIco {
                            name_label: "triangle",
                            hover: "/imgs/tri-hover.png",
                            normal: "/imgs/tri.png",
                        },
                        0.8,
                        "white",
                        "white",
                        "/imgs/logo.png",
                        "white",
                        "red",
                        "#ffffff50",
                        "#ffffffaa",
                        "transparent",
                        HeaderIco {
                            name_label: "history",
                            normal: "/imgs/history.png",
                            hover: "/imgs/history-hover.png",
                        },
                        HeaderIco {
                            name_label: "download",
                            normal: "/imgs/download.png",
                            hover: "/imgs/download-hover.png",
                        },
                        HeaderIco {
                            name_label: "start",
                            normal: "/imgs/ls.png",
                            hover: "/imgs/ls-hover.png",
                        },
                    )
                }
                _ => {
                    format!(
                        "{}; --triangle-opacity: {}; --right-header: {}; --right-header-hover: {}; --logo-url: url({}); --search-icon: {}; --search-icon-hover: {}; --search-bg: {}; --search-bg-hover: {}; --search-border: {}; {}; {}; {}",
                        HeaderIco {
                            name_label: "triangle",
                            hover: "/imgs/tri-white-hover.png",
                            normal: "/imgs/tri-white.png",
                        },
                        1,
                        "gray",
                        "#f40",
                        "/imgs/logo2.png",
                        "gray",
                        "#f80",
                        "#e5e7eb",
                        "#fff",
                        "#f80",
                        HeaderIco {
                            name_label: "history",
                            normal: "/imgs/history-white.png",
                            hover: "/imgs/history-white-hover.png",
                        },
                        HeaderIco {
                            name_label: "download",
                            normal: "/imgs/download-white.png",
                            hover: "/imgs/download-white-hover.png",
                        },
                        HeaderIco {
                            name_label: "start",
                            normal: "/imgs/ls-white.png",
                            hover: "/imgs/ls-white-hover.png",
                        },
                    )
                }
            }
            class=move || match (matched.get().path(), stickied.get()) {
                ("/", false) => css::hy_header.to_string(),
                ("/", true) => format!("{} {}", css::hy_header, css::sticky),
                _ => format!("{} {}", css::hy_header, css::light),
            }
        >
            <div class=css::inner_header>
                <a href="/" class=css::logo />
                <nav class=format!("{}", css::navs_clsx)>
                    <A href="">首页</A>
                    <A href="l">直播</A>
                    <A href="g">
                        分类 <i />
                        <div
                            on:click:target=move |ev| {
                                ev.prevent_default();
                            }
                            data-active
                            class=css::cate_pop_clsx
                        >
                            <For
                                each=|| CATEGROY.clone().into_iter()
                                key=|item| item.0
                                let((title, tags))
                            >
                                <h1>{title}</h1>
                                <div class=css::tags>
                                    <For
                                        each=move || tags.clone().into_iter()
                                        key=|tag| *tag
                                        let(tag)
                                    >
                                        <span class=css::tag>{tag}</span>
                                    </For>
                                </div>
                            </For>
                            <div class=css::more>更多 ></div>
                            <Ad />
                        </div>
                    </A>
                    <A href="m">赛事<i /></A>
                    <A href="video">视频<i /></A>
                    <A href="game">游戏<i /></A>

                    <Form method="GET" action="/search">
                        <input
                            type="text"
                            name="hsk"
                            placeholder="寻寻觅觅"
                            class=css::search_clsx
                        />
                        <svg
                            class=css::search_icon_clsx
                            viewBox="0 0 1024 1024"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path d="M909.6 854.5L649.9 594.8C690.2 542.7 712 479 712 412c0-80.2-31.3-155.4-87.9-212.1-56.6-56.7-132-87.9-212.1-87.9s-155.5 31.3-212.1 87.9C143.2 256.5 112 331.8 112 412c0 80.1 31.3 155.5 87.9 212.1C256.5 680.8 331.8 712 412 712c67 0 130.6-21.8 182.7-62l259.7 259.6c3.2 3.2 8.4 3.2 11.6 0l43.6-43.5c3.2-3.2 3.2-8.4 0-11.6zM570.4 570.4C528 612.7 471.8 636 412 636s-116-23.3-158.4-65.6C211.3 528 188 471.8 188 412s23.3-116.1 65.6-158.4C296 211.3 352.2 188 412 188s116.1 23.2 158.4 65.6S636 352.2 636 412s-23.3 116.1-65.6 158.4z"></path>
                        </svg>
                        <input type="submit" value="" />
                    </Form>
                </nav>
                <ul class=css::nav_right_clsx>
                    <li class=css::start>开播</li>
                    <li class=css::download>
                        下载 <div class=css::download_pop_clsx>
                            <div class=css::top_line>
                                <For
                                    each=move || DOWNLOADS.clone().into_iter()
                                    key=|item| item.title
                                    let(item)
                                >
                                    <div class=css::download_item_clsx>
                                        <h1>{item.title}</h1>
                                        <h2>{item.description}</h2>
                                        <img
                                            src=move || item.img_url
                                            width=item.size.0
                                            height=item.size.1
                                            alt="扫码下载"
                                        />
                                        <a href="#">{item.type_label}</a>
                                    </div>
                                </For>
                            </div>
                            <div class=css::bottom_line>
                                <img width="19" height="16" src="/imgs/icon-anchor.png" />
                                <span>虎牙主播端</span>
                                <span>简单易用一件开通</span>
                            </div>
                        </div>
                    </li>
                    <li class=css::history>历史</li>
                    <li class=css::task>
                        任务 <div class=css::task_pop_clsx>
                            <div class=css::inner_container>
                                <div class=css::top_line>
                                    "登录领取积分，兑换超多福利"
                                    // linear-gradient(221deg, #ffe44d 0%, #ffd736 100%)
                                    <button class=css::login_now>立即登录</button>
                                </div>
                                <div class=css::task_list>
                                    <For
                                        each=|| TASKS.clone().into_iter()
                                        key=|item| {
                                            format!(
                                                "{}_{}_{}",
                                                item.img_url,
                                                item.cost_value,
                                                item.value,
                                            )
                                        }
                                        let(item)
                                    >
                                        <div class=css::task>
                                            <img src=item.img_url />
                                            <span>{item.cost_value}</span>
                                            <div class=css::task_item_clsx>{item.value}</div>
                                        </div>
                                    </For>
                                </div>
                            </div>
                        </div>
                    </li>
                    <li class=css::login>
                        登录 <div class=css::login_pop_clsx>
                            <h1>登陆后可享受:</h1>
                            <ul class=css::login_tips_clsx>
                                <li>蓝光6M高清画质</li>
                                <li>独家赛事超前关注</li>
                                <li>多元玩法精彩互动</li>
                            </ul>
                            <div class=css::login_btn>登录</div>
                            <a href="" class=css::register_clsx>
                                点我注册
                            </a>
                        </div>
                    </li>
                </ul>
            </div>
        </header>
    }
}
