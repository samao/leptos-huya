use leptos::prelude::*;
use leptos_router::{
    components::{Form, A},
    hooks::use_url,
};
#[cfg(feature = "hydrate")]
use leptos_use::use_window_scroll;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, sync::LazyLock};

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
                <div class="relative">
                    <img src=result width="274" height="65" />
                    <span
                        class="absolute top-0 right-0 hover:text-white size-5 bg-gray-300/50 text-black/20"
                        on:click=move |_| show_ad.set(false)
                    >
                        x
                    </span>
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
                ("/", false) => "min-w-[980px] w-full text-white duration-500",
                ("/", true) => {
                    "min-w-[980px] w-full text-[#555] duration-500 bg-white sticky top-0 z-20"
                }
                _ => "min-w-[980px] w-full text-[#555] bg-white duration-500",
            }
        >
            <div class="flex justify-start items-center mx-auto whitespace-nowrap w-[980px] min-[1440px]:w-[1220px] h-[60px] text-[16px]/[32px]">
                <a
                    href="/"
                    class="inline-block object-contain flex-none mr-5 h-9 bg-center bg-[image:var(--logo-url)] w-30 bg-size-[100%]"
                />
                <nav class="flex flex-auto gap-x-2 *:[a]:h-8 *:[a]:leading-8 *:hover:bg-[#ff9600] *:hover:text-white *:rounded-2xl *:px-4 *:hover:[form]:bg-transparent *:[form]:relative *:[form]:mr-2 *:[form]:flex *:[form]:items-center *:[form]:px-0 *:aria-[current]:bg-[#ff9600] *:aria-[current]:text-white *:aria-[current]:has-[i]:*:[i]:bg-[image:var(--triangle-hover)] **:[i]:inline-block *:flex *:items-center **:[i]:duration-200 *:gap-x-2 *:has-[i]:hover:*:[i]:rotate-180 *:relative *:has-[i]:hover:*:data-[active]:flex **:[i]:w-[9px] **:[i]:h-[5px] *:has-[i]:hover:*:[i]:opacity-100 *:has-[i]:*:[i]:opacity-[var(--triangle-opacity)] *:has-[i]:hover:*:[i]:bg-[image:var(--triangle-hover)] *:has-[i]:*:[i]:bg-[image:var(--triangle-icon)]">
                    <A href="">首页</A>
                    <A href="l">直播</A>
                    <A href="g">
                        分类 <i />
                        <div
                            on:click:target=move |ev| {
                                ev.prevent_default();
                            }
                            data-active
                            class="hidden absolute left-1/2 top-full z-10 flex-col gap-y-2 p-4 pb-6 bg-white rounded-md -translate-x-1/2 translate-y-2 cursor-default before:h-2 before:w-full before:absolute before:-top-2 text-[12px]/[20px] text-[#333] w-[304px]"
                        >
                            <For
                                each=|| CATEGROY.clone().into_iter()
                                key=|item| item.0
                                let((title, tags))
                            >
                                <h1 class="font-bold text-left text-[14px]">{title}</h1>
                                <div class="grid grid-cols-3 gap-2 *:rounded-xl *:border">
                                    <For
                                        each=move || tags.clone().into_iter()
                                        key=|tag| *tag
                                        let(tag)
                                    >
                                        <span class="inline-block text-center hover:text-white border-[#e3e7e8] hover:bg-[#ff9600]">
                                            {tag}
                                        </span>
                                    </For>
                                </div>
                            </For>
                            <div class="text-center rounded-3xl hover:text-white bg-gray-300/80 text-[12px]/[28px] hover:bg-[#ff9600]">
                                更多 >
                            </div>
                            <Ad />
                        </div>
                    </A>
                    // <a
                    // href="m"
                    // class="inline-block bg-center bg-no-repeat bg-contain bg-[url(https://diy-assets.msstatic.com/header-match-icon/icon.png)] w-[74px] hover:bg-transparent!"
                    // ></a>
                    <A href="m">赛事<i /></A>
                    <A href="video">视频<i /></A>
                    <A href="game">游戏<i /></A>

                    <Form method="GET" action="/search">
                        <input
                            type="text"
                            name="hsk"
                            placeholder="寻寻觅觅"
                            class="pr-10 pl-4 rounded-2xl border border-transparent min-[1440px]:w-[140px] w-[100px] text-xs/[32px] bg-[var(--search-bg)] peer placeholder:text-[#555]/70 hover:border-[var(--search-border)] hover:bg-[var(--search-bg-hover)] focus:border-[var(--search-border)] focus:bg-[var(--search-bg-hover)] focus:text-[#555] focus-visible:outline-0"
                        />
                        <svg
                            class="absolute right-2 size-6 fill-[var(--search-icon)] peer-focus:fill-[var(--search-icon-hover)] peer-hover:fill-[var(--search-icon-hover)]"
                            viewBox="0 0 1024 1024"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path d="M909.6 854.5L649.9 594.8C690.2 542.7 712 479 712 412c0-80.2-31.3-155.4-87.9-212.1-56.6-56.7-132-87.9-212.1-87.9s-155.5 31.3-212.1 87.9C143.2 256.5 112 331.8 112 412c0 80.1 31.3 155.5 87.9 212.1C256.5 680.8 331.8 712 412 712c67 0 130.6-21.8 182.7-62l259.7 259.6c3.2 3.2 8.4 3.2 11.6 0l43.6-43.5c3.2-3.2 3.2-8.4 0-11.6zM570.4 570.4C528 612.7 471.8 636 412 636s-116-23.3-158.4-65.6C211.3 528 188 471.8 188 412s23.3-116.1 65.6-158.4C296 211.3 352.2 188 412 188s116.1 23.2 158.4 65.6S636 352.2 636 412s-23.3 116.1-65.6 158.4z"></path>
                        </svg>
                        <input
                            type="submit"
                            value=""
                            class="absolute right-2 bg-red-400 opacity-0 size-6"
                        />
                    </Form>
                </nav>
                <ul class="flex gap-x-6 items-center select-none text-[var(--right-header)] *:[li]:hover:text-[var(--right-header-hover)] *:[li]:opacity-60 *:[li]:hover:opacity-100 text-xs/1.5 *:relative *:flex *:flex-col *:justify-center *:before:size-[26px] *:gap-1.5 *:before:bg-cover *:before:bg-no-repeat *:bg-center *:[li]:hover:*:[div]:block">
                    <li class="before:bg-[image:var(--start-icon)] hover:before:bg-[image:var(--start-hover)]">
                        开播
                    </li>
                    <li class="before:bg-[image:var(--download-icon)] hover:before:bg-[image:var(--download-hover)]">
                        下载
                        <div class="hidden absolute -left-2.5 top-full z-10 mt-3 bg-white rounded-md -translate-x-1/2 w-[436px] text-[#666] before:h-3 before:-top-3 before:left-0 before:w-full before:absolute">
                            <div class="flex justify-between p-5 leading-3.5">
                                <For
                                    each=move || DOWNLOADS.clone().into_iter()
                                    key=|item| item.title
                                    let(item)
                                >
                                    <div class="flex flex-col gap-y-0 justify-between items-center first:after:bg-[#e5e5e5] first:after:absolute first:after:w-[1px] first:after:h-full first:after:-right-4 hover:*:[a]:text-[#f40]">
                                        <h1 class="font-bold">{item.title}</h1>
                                        <h2 class="py-1">{item.description}</h2>
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
                            <div class="flex gap-x-2 items-center pl-5 bg-[#f4f4f4] text-[12px]/[40px]">
                                <img width="19" height="16" src="/imgs/icon-anchor.png" />
                                <span class="font-bold">虎牙主播端</span>
                                <span>简单易用一件开通</span>
                            </div>
                        </div>
                    </li>
                    <li class="before:bg-[image:var(--history-icon)] hover:before:bg-[image:var(--history-hover)]">
                        历史
                    </li>
                    <li class="before:bg-[url(/imgs/cal-3.png)]">
                        任务
                        <div class="hidden absolute top-full z-10 p-3 mt-3 bg-top bg-no-repeat rounded-md -translate-x-1/2 w-[375px] min-h-[182px] bg-[#f5f6f7] bg-[url(/imgs/bg.png)] text-[12px]/[20px] text-[#666] -left-[100px] before:h-3 before:-top-3 before:left-0 before:w-full before:absolute">
                            <div class="p-4 bg-white rounded-md">
                                <div class="flex justify-between mb-2.5">
                                    "登录领取积分，兑换超多福利"
                                    // linear-gradient(221deg, #ffe44d 0%, #ffd736 100%)
                                    <button class="py-0.5 px-2.5 font-bold bg-linear-[221deg] from-0% from-[#ffe44d] to-100% to-[#ffd736] rounded-[44px]">
                                        立即登录
                                    </button>
                                </div>
                                <div class="flex justify-between">
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
                                        <div class="flex flex-col gap-y-2.5 items-center">
                                            <img src=item.img_url class="w-9 h-auto" />
                                            <span>{item.cost_value}</span>
                                            <div class="flex gap-x-1 items-center px-1.5 font-bold rounded-3xl border border-gray-300 text-[12px]/[30px] before:size-3.5 before:bg-cover before:bg-center before:bg-[url(/imgs/icon.png)]">
                                                {item.value}
                                            </div>
                                        </div>
                                    </For>
                                </div>
                            </div>
                        </div>
                    </li>
                    <li class="rounded-full ring-2 border-white/35 text-[14px]/[34px] size-[34px] before:hidden">
                        登录
                        <div class="hidden absolute top-full left-full z-10 pt-2 mt-3 bg-white rounded-md -translate-x-full text-[#333] px-[25px] pb-[13px] before:h-3 before:-top-3 before:left-0 before:w-[200px] before:absolute">
                            <h1 class="font-bold text-left">登陆后可享受:</h1>
                            <ul class="flex flex-col text-left *:[li]:flex *:[li]:items-center *:[li]:gap-x-2.5 *:[li]:before:size-[18px] *:[li]:before:bg-cover *:[li]:before:bg-no-repeat">
                                <li class="before:bg-[url(/imgs/a.png)]">蓝光6M高清画质</li>
                                <li class="before:bg-[url(/imgs/b.png)]">
                                    独家赛事超前关注
                                </li>
                                <li class="before:bg-[url(/imgs/c.png)]">
                                    多元玩法精彩互动
                                </li>
                            </ul>
                            <div class="my-2 text-white rounded-3xl bg-[#ffa900] text-[14px]/[30px] hover:opacity-85">
                                登录
                            </div>
                            <a
                                href=""
                                class="flex gap-x-1 justify-center text-xs text-sky-500 [&::before,&::after]:text-gray-400/40 before:content-['>>'] after:content-['<<']"
                            >
                                点我注册
                            </a>
                        </div>
                    </li>
                </ul>
            </div>
        </header>
    }
}
