use leptos::prelude::*;
use leptos_router::components::{Form, A};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::LazyLock};

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
        ("虎牙APP", "独家赛事随时享", "https://a.msstatic.com/huya/hd/h5/header/components/Download/img/h-code.1d32b4ae284a3920100014734278a934.png", "扫码下载", 100, 100).into(),
        ("虎牙PC客户端", "畅想蓝光臻画质", "https://a.msstatic.com/huya/hd/h5/header/components/Download/img/h-pc2.ce21ce5e431fc4b57e21a4b97566759d.png", "点击下载", 108, 108).into(),
        ("虎牙TV电视端", "巨幕蓝光沉浸体验", "https://a.msstatic.com/huya/hd/h5/header/components/Download/img/h-TV.b00384c2386a51590fae10da845d8ed0.png", "点击下载",  108, 1088).into()
    ]
});

static TASKS: LazyLock<Vec<Task>> = LazyLock::new(|| {
    vec![Task {
        img_url: "",
        cost_value: 60,
        value: 6000,
    }]
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

impl<'a> From<(&'a str, &'a str, &'a str, &'a str, u32, u32)> for DownloadItem<'a> {
    fn from(
        (title, description, img_url, type_label, width, height): (
            &'a str,
            &'a str,
            &'a str,
            &'a str,
            u32,
            u32,
        ),
    ) -> Self {
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
struct Task<'a> {
    img_url: &'a str,
    cost_value: u32,
    value: u32,
}

#[component]
fn Ad() -> impl IntoView {
    let show_ad = RwSignal::new(true);

    view! {
        <Show when=move || show_ad.get() fallback=|| "">
            <div class="relative">
                <img
                    src="https://livewebbs2.msstatic.com/huya_1716264051_content.gif"
                    width="274"
                    height="65"
                />
                <span
                    class="size-5 bg-gray-300/50 text-black/20 absolute top-0 right-0 hover:text-white"
                    on:click=move |_| show_ad.set(false)
                >
                    x
                </span>
            </div>
        </Show>
    }
}
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="w-full">
            <div class="flex whitespace-nowrap w-[980px] min-[1440px]:w-[1220px] mx-auto h-[60px] text-[16px]/[32px] items-center justify-start text-white">
                <a
                    href="/"
                    class="inline-block flex-none mr-5 bg-[url(https://a.msstatic.com/huya/hd/h5/static-source/main/logo.png)] w-30 h-9 bg-size-[100%] bg-center object-contain"
                />
                <nav class="flex flex-auto gap-x-2 *:hover:bg-[#ff9600] *:rounded-2xl *:px-4 *:hover:[form]:bg-transparent *:[form]:relative *:[form]:mr-2 *:[form]:flex *:[form]:items-center *:[form]:px-0
                *:aria-[current]:bg-[#ff9600] **:[i]:inline-block *:flex *:items-center **:[i]:duration-200 *:gap-x-2 
                *:has-[i]:hover:*:[i]:rotate-180 *:relative *:has-[i]:hover:*:data-[active]:flex **:[i]:w-[9px] **:[i]:h-[5px] **:[i]:bg-[url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAkAAAAFCAYAAACXU8ZrAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAABmJLR0QAAAAAAAD5Q7t/AAAACXBIWXMAAAsSAAALEgHS3X78AAAALUlEQVQI12P8////fwYCgJGBgYEBn0JGRkZGRhgHm0KYPCOyILJCZAMwADYTAdkUE/0YEgvwAAAAAElFTkSuQmCC)]">
                    <A href="">首页</A>
                    <A href="l">直播</A>
                    <A href="g">
                        分类 <i />
                        <div
                            on:click:target=move |ev| {
                                ev.prevent_default();
                            }
                            data-active
                            class="hidden cursor-default before:h-2 before:w-full before:absolute before:-top-2 flex-col gap-y-2 text-[12px]/[20px] text-[#333] w-[304px] p-4 pb-6 bg-white rounded-md absolute -translate-x-1/2 translate-y-2 z-10 left-1/2 top-full"
                        >
                            <For
                                each=|| CATEGROY.clone().into_iter()
                                key=|item| item.0
                                let((title, tags))
                            >
                                <h1 class="text-[14px] text-left font-bold">{title}</h1>
                                <div class="grid grid-cols-3 gap-2 *:rounded-xl *:border">
                                    <For
                                        each=move || tags.clone().into_iter()
                                        key=|tag| *tag
                                        let(tag)
                                    >
                                        <span class="inline-block text-center border-[#e3e7e8] hover:text-white hover:bg-[#ff9600]">
                                            {tag}
                                        </span>
                                    </For>
                                </div>
                            </For>
                            <div class="rounded-3xl bg-gray-300/80 text-center text-[12px]/[28px]  hover:text-white hover:bg-[#ff9600]">
                                更多 >
                            </div>
                            <Ad />
                        </div>
                    </A>
                    <a
                        href="m"
                        class="inline-block bg-[url(https://diy-assets.msstatic.com/header-match-icon/icon.png)] w-[74px] bg-center bg-contain bg-no-repeat hover:bg-transparent!"
                    ></a>
                    <A href="video">视频<i /></A>
                    <A href="game">游戏<i /></A>

                    <Form method="GET" action="/search">
                        <input
                            type="text"
                            name="hsk"
                            placeholder="寻寻觅觅"
                            class="min-[1440px]:w-[140px] w-[100px] text-xs/[32px] rounded-2xl bg-white/40 peer placeholder:text-[#555]/70 focus:bg-white/80 hover:bg-white/80 focus-visible:outline-none pl-4 pr-10 focus:text-[#555] focus:border-none "
                        />
                        <svg
                            class="absolute right-2 size-6 fill-white peer-focus:fill-red-500 peer-hover:fill-red-500"
                            viewBox="0 0 1024 1024"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path d="M909.6 854.5L649.9 594.8C690.2 542.7 712 479 712 412c0-80.2-31.3-155.4-87.9-212.1-56.6-56.7-132-87.9-212.1-87.9s-155.5 31.3-212.1 87.9C143.2 256.5 112 331.8 112 412c0 80.1 31.3 155.5 87.9 212.1C256.5 680.8 331.8 712 412 712c67 0 130.6-21.8 182.7-62l259.7 259.6c3.2 3.2 8.4 3.2 11.6 0l43.6-43.5c3.2-3.2 3.2-8.4 0-11.6zM570.4 570.4C528 612.7 471.8 636 412 636s-116-23.3-158.4-65.6C211.3 528 188 471.8 188 412s23.3-116.1 65.6-158.4C296 211.3 352.2 188 412 188s116.1 23.2 158.4 65.6S636 352.2 636 412s-23.3 116.1-65.6 158.4z"></path>
                        </svg>
                        <input
                            type="submit"
                            value=""
                            class="absolute right-2 size-6 bg-red-400 opacity-0"
                        />
                    </Form>
                </nav>
                <ul class="flex gap-x-6 text-xs/1.5 items-center *:relative *:flex *:flex-col *:justify-center
                *:before:size-[26px] *:gap-1.5 *:before:bg-cover *:before:bg-no-repeat *:bg-center *:[li]:hover:opacity-100 *:nth-[-n+3]:opacity-60
                *:[li]:hover:*:[div]:block">
                    <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/NavItem/img/ls-2.9113b9d316856ca1d795c0e54079d940.png)]">
                        开播
                    </li>
                    <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/NavItem/img/download-2.c9310be282ee8f2196da396cf89c916b.png)]">
                        下载
                        <div class="hidden w-[436px] z-10 bg-white rounded-md text-[#666] absolute -translate-x-1/2 top-full mt-3 left-1/2 before:h-3 before:-top-3
                        before:left-0 before:w-full before:absolute">
                            <div class="flex justify-between p-5">
                                <For
                                    each=move || DOWNLOADS.clone().into_iter()
                                    key=|item| item.title
                                    let(item)
                                >
                                    <div class="flex flex-col items-center gap-y-2 justify-between hover:*:[a]:text-[#f40]
                                    first:after:bg-[#e5e5e5] first:after:absolute first:after:w-[1px] first:after:h-full first:after:-right-4">
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
                            <div class="flex gap-x-2 items-center bg-[#f4f4f4] text-[12px]/[40px] pl-5">
                                <img
                                    width="19"
                                    height="16"
                                    src="https://a.msstatic.com/huya/hd/h5/header/components/Download/img/icon-anchor.405345a5f556023d0041a4c4defa1fac.png"
                                />
                                <span class="font-bold">虎牙主播端</span>
                                <span>简单易用一件开通</span>
                            </div>
                        </div>
                    </li>
                    <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/NavItem/img/history-2.6157ee9c44045989cf42ff47033a592f.png)]">
                        历史
                    </li>
                    <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/NavItem/img/cal-3.e73b55e606fac48daf82cf9982d6ef25.png)]">
                        任务 <div class="hidden">
                            <div>
                                <div class="flex justify-between">
                                    "登录领取积分，兑换超多福利"
                                    <button>立即登录</button>
                                </div>
                                <For
                                    each=|| TASKS.clone().into_iter()
                                    key=|item| item.img_url
                                    let(item)
                                >
                                    <div class="flex flex-col">
                                        <img src=item.img_url />
                                        <span>{item.cost_value}"点券"</span>
                                        <div class="flex before:content-['*'] items-center gap-x-1 rounded-3xl">
                                            {item.value}
                                        </div>
                                    </div>
                                </For>
                            </div>
                        </div>
                    </li>
                    <li class="border-white/35 ring-2 rounded-full text-[14px]/[34px] size-[34px] before:hidden">
                        登录
                        <div class="hidden rounded-md text-[#333] px-[25px] pb-[13px] pt-2 bg-white absolute -translate-x-full left-full top-full mt-3 z-10
                        before:h-3 before:-top-3 before:left-0 before:w-[200px] before:absolute">
                            <h1 class="text-left font-bold">登陆后可享受:</h1>
                            <ul class="flex flex-col text-left *:[li]:flex *:[li]:items-center *:[li]:gap-x-2.5 *:[li]:before:size-[18px] *:[li]:before:bg-cover *:[li]:before:bg-no-repeat">
                                <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/Login/img/a.bd84283473df965f75a07ee3f1933e57.png)]">
                                    蓝光6M高清画质
                                </li>
                                <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/Login/img/b.101344f1440dd469268eb425d1ae573a.png)]">
                                    独家赛事超前关注
                                </li>
                                <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/Login/img/c.23d9778a52ccd563d5c7ebbbf525235f.png)]">
                                    多元玩法精彩互动
                                </li>
                            </ul>
                            <div class="bg-[#ffa900] text-[14px]/[30px] rounded-3xl hover:opacity-85 text-white my-2">
                                登录
                            </div>
                            <a
                                href=""
                                class="text-sky-500 text-xs flex justify-center gap-x-1 [&::before,&::after]:text-gray-400/40 before:content-['>>'] after:content-['<<']"
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
